#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream},
    parse2,
    token::Paren,
    Error as SynError, Expr, Result as SynResult, Token,
};
use timrs_macro_utils::{
    error::unexpected_token_message,
    parse::{accumulate_while, LookaheadToken},
};

#[derive(Debug, PartialEq)]
enum OperatorToken {
    After,
    Before,
}

impl OperatorToken {
    fn resolve<F>(parsed: Self, lhs: F, rhs: F) -> impl FnOnce(TokenStream2) -> TokenStream2
    where
        F: FnOnce(TokenStream2) -> TokenStream2,
    {
        move |input| match parsed {
            Self::After => lhs(rhs(input)),
            Self::Before => rhs(lhs(input)),
        }
    }
}

impl Parse for OperatorToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![<]) && input.peek2(Token![|]) {
            input.parse::<Token![<]>()?;
            input.parse::<Token![|]>()?;

            SynResult::Ok(Self::After)
        } else if input.peek(Token![|]) && input.peek2(Token![>]) {
            input.parse::<Token![|]>()?;
            input.parse::<Token![>]>()?;

            SynResult::Ok(Self::Before)
        } else {
            SynResult::Err(SynError::new(
                input.span(),
                unexpected_token_message("`Operator (<| or |>)`", &input.to_string()),
            ))
        }
    }
}

impl LookaheadToken for OperatorToken {
    const SIZE: usize = 2;
}

struct FunctionToken {
    value: Expr,
}

impl FunctionToken {
    fn resolve(parsed: Self) -> impl FnOnce(TokenStream2) -> TokenStream2 {
        move |input| {
            let value = parsed.value;

            quote! { (#value)(#input) }
        }
    }
}

impl Parse for FunctionToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        SynResult::Ok(Self {
            value: input.parse::<Expr>()?,
        })
    }
}

enum OperandToken {
    Function(FunctionToken),
    Operation(OperationToken),
    Parenthesized(Box<OperandToken>),
}

impl OperandToken {
    fn resolve(parsed: Self) -> impl FnOnce(TokenStream2) -> TokenStream2 {
        move |input| match parsed {
            Self::Function(value) => FunctionToken::resolve(value)(input),
            Self::Operation(value) => OperationToken::resolve(value)(input),
            Self::Parenthesized(value) => Self::resolve(*value)(input),
        }
    }
}

impl Parse for OperandToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Paren) {
            let fork = input.fork();
            let content;

            parenthesized!(content in fork);

            if fork.is_empty() {
                input.advance_to(&fork);

                return content.parse::<Self>().map(Box::new).map(Self::Parenthesized);
            }
        }

        input.step(|cursor| {
            let mut is_operation = false;
            let (stream, next) = accumulate_while(*cursor, |current| {
                if !is_operation {
                    is_operation = OperatorToken::lookahead(current).is_ok();
                }

                !current.eof()
            })?;

            if is_operation {
                parse2::<OperationToken>(stream)
                    .map(Self::Operation)
                    .map(|value| (value, next))
            } else {
                parse2::<FunctionToken>(stream)
                    .map(Self::Function)
                    .map(|value| (value, next))
            }
        })
    }
}

struct OperationToken {
    lhs: Box<OperandToken>,
    operator: OperatorToken,
    rhs: Box<OperandToken>,
}

impl OperationToken {
    fn resolve(parsed: Self) -> impl FnOnce(TokenStream2) -> TokenStream2 {
        move |input| {
            OperatorToken::resolve(
                parsed.operator,
                OperandToken::resolve(*parsed.lhs),
                OperandToken::resolve(*parsed.rhs),
            )(input)
        }
    }
}

impl Parse for OperationToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Paren) {
            let fork = input.fork();
            let content;

            parenthesized!(content in fork);

            if fork.is_empty() {
                input.advance_to(&fork);

                return content.parse::<Self>();
            }
        }

        input.step(|lhs_cursor| {
            let (lhs, operator_cursor) =
                accumulate_while(*lhs_cursor, |current| OperatorToken::lookahead(current).is_err())?;
            let (operator, rhs_cursor) = OperatorToken::lookahead(operator_cursor)?;
            let (rhs, next) = accumulate_while(rhs_cursor, |current| !current.eof())?;

            SynResult::Ok((
                Self {
                    lhs: parse2::<OperandToken>(lhs).map(Box::new)?,
                    operator,
                    rhs: parse2::<OperandToken>(rhs).map(Box::new)?,
                },
                next,
            ))
        })
    }
}

#[derive(Debug, PartialEq)]
struct InsertToken;

impl Parse for InsertToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        if input.peek(Token![-]) && input.peek2(Token![>]) && input.peek3(Token![>]) {
            input.parse::<Token![-]>()?;
            input.parse::<Token![>]>()?;
            input.parse::<Token![>]>()?;

            SynResult::Ok(Self {})
        } else {
            SynResult::Err(SynError::new(
                input.span(),
                unexpected_token_message("`Insert (->>)`", &input.to_string()),
            ))
        }
    }
}

impl LookaheadToken for InsertToken {
    const SIZE: usize = 3;
}

struct InputToken {
    value: Expr,
}

impl Parse for InputToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input.step(|cursor| {
            let (stream, next) = accumulate_while(*cursor, |current| InsertToken::lookahead(current).is_err())?;

            parse2::<Expr>(stream).map(|value| (Self { value }, next))
        })
    }
}

struct PipeToken {
    input: InputToken,
    operation: OperationToken,
}

impl Parse for PipeToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_token = input.parse::<InputToken>()?;
        input.parse::<InsertToken>()?;
        let operation_token = input.parse::<OperationToken>()?;

        SynResult::Ok(Self {
            input: input_token,
            operation: operation_token,
        })
    }
}

fn impl_pipe(tokens: TokenStream2) -> TokenStream2 {
    parse2::<PipeToken>(tokens).map_or_else(SynError::into_compile_error, |pipe_token| {
        OperationToken::resolve(pipe_token.operation)(pipe_token.input.value.to_token_stream())
    })
}

/// Macro providing the custom piping binary operators `before (|>)` and `after (<|)`, operations can be parenthesized and are left-associative.
///
/// # Examples
/// ## After Operator (`<|`):
/// ```
/// use timrs_pipe_macro::pipe;
///
/// fn increment(x: i32) -> i32 { x + 1 }
/// fn to_string(x: i32) -> String { x.to_string() }
/// fn greet(x: String) -> String { format!("HELLO {x}!") }
///
/// assert_eq!(
///     pipe! { 1 ->> greet <| to_string <| increment },
///     "HELLO 2!"
/// );
/// ```
///
/// ## Before Operator (`|>`):
///
/// ```
/// use timrs_pipe_macro::pipe;
///
/// fn increment(x: i32) -> i32 { x + 1 }
/// fn to_string(x: i32) -> String { x.to_string() }
/// fn greet(x: String) -> String { format!("HELLO {x}!") }
///
/// assert_eq!(
///     pipe! { 1 ->> increment |> to_string |> greet },
///     "HELLO 2!"
/// );
/// ```
///
/// # Grammar
/// The grammar rules for the `pipe!` macro are the following:
///
/// ## Tokens
/// ```text
/// e_e        := (end of input)
/// e_function := (any valid callable rust expression, i.e. can be called with the following syntax `f ()`)
/// e_input    := (any valid rust expression that can be used as a function input)
/// s_insert   := "->>"
/// s_p_left   := "("
/// s_p_right  := ")"
/// s_o_after  := "<|"
/// s_o_before := "|>"
/// ```
///
/// ## Rules
/// ```text
/// pipe                    := e_input s_insert operation e_e
/// operation               := parenthesized_operation | operand operator operand
/// parenthesized_operation := s_p_left operation s_p_right
/// operator                := s_o_after | s_o_before
/// operand                 := parenthesized_operand | operation | e_function
/// parenthesized_operand   := s_p_left operand s_p_right
/// ```
#[proc_macro]
pub fn pipe(tokens: TokenStream) -> TokenStream { impl_pipe(tokens.into()).into() }

#[cfg(test)]
mod tests {
    #[test]
    fn should_correctly_parse_after_operator_token() -> Result<(), String> {
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::OperatorToken;

        let expected = OperatorToken::After;
        let input = "<|";
        let output = parse_test::<OperatorToken>(build_tokens(input)).map_err(|error| error.to_string())?;

        assert_eq!(output, expected, "Testing `OperatorToken` successful parsing: `After`");

        Result::Ok(())
    }

    #[test]
    fn should_correctly_parse_before_operator_token() -> Result<(), String> {
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::OperatorToken;

        let expected = OperatorToken::Before;
        let input = "|>";
        let output = parse_test::<OperatorToken>(build_tokens(input)).map_err(|error| error.to_string())?;

        assert_eq!(output, expected, "Testing `OperatorToken` successful parsing: `Before`");

        Result::Ok(())
    }

    #[test]
    fn should_correctly_fail_parsing_operator_token() -> Result<(), String> {
        use timrs_macro_utils::{
            error::unexpected_token_message,
            test::{build_tokens, parse_test, pretty_print},
        };

        use super::OperatorToken;

        let expected_token = "`Operator (<| or |>)`";
        let got_token = "INVALID_INPUT";

        let expected = format!(
            "::core::compile_error!{{\"{}\"}}",
            unexpected_token_message(expected_token, got_token)
        );
        let input = got_token;
        let output = parse_test::<OperatorToken>(build_tokens(input)).unwrap_err();

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(&expected)),
            "Testing `OperatorToken` unsuccessful parsing"
        );

        Result::Ok(())
    }

    #[test]
    fn should_have_the_correct_lookahead_token_size_for_operator_token() {
        use timrs_macro_utils::parse::LookaheadToken;

        use super::OperatorToken;

        let expected = 2_usize;

        assert_eq!(OperatorToken::SIZE, expected)
    }

    #[test]
    fn should_correctly_parse_function_token() -> Result<(), String> {
        use quote::ToTokens;
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::FunctionToken;

        let input_closure = "| x | x + 1";
        let input_identifier = "a";
        let input_parenthesized = "(a)";
        let output_closure =
            parse_test::<FunctionToken>(build_tokens(input_closure)).map_err(|error| error.to_string())?;
        let output_identifier =
            parse_test::<FunctionToken>(build_tokens(input_identifier)).map_err(|error| error.to_string())?;
        let output_parenthesized =
            parse_test::<FunctionToken>(build_tokens(input_parenthesized)).map_err(|error| error.to_string())?;

        assert_eq!(
            output_closure.value.to_token_stream().to_string(),
            input_closure,
            "Testing `FunctionToken` successful parsing: Closure"
        );
        assert_eq!(
            output_identifier.value.to_token_stream().to_string(),
            input_identifier,
            "Testing `FunctionToken` successful parsing: Identifier"
        );
        assert_eq!(
            output_parenthesized.value.to_token_stream().to_string(),
            input_parenthesized,
            "Testing `FunctionToken` successful parsing: Parenthesized"
        );

        Result::Ok(())
    }

    #[test]
    fn should_correctly_fail_parsing_function_token() -> Result<(), String> {
        use quote::ToTokens;
        use timrs_macro_utils::test::{build_tokens, parse_test, pretty_print};

        use super::FunctionToken;

        let input_not_expression = "+";
        let output_not_expression = parse_test::<FunctionToken>(build_tokens(input_not_expression))
            .map(|value| value.value.to_token_stream().to_string())
            .unwrap_err();

        assert_eq!(
            pretty_print(output_not_expression),
            pretty_print(build_tokens("::core::compile_error!{\"expected an expression\"}")),
            "Testing `FunctionToken` unsuccessful parsing: Not an expression"
        );

        let input_eof = "";
        let output_eof = parse_test::<FunctionToken>(build_tokens(input_eof))
            .map(|value| value.value.to_token_stream().to_string())
            .unwrap_err();

        assert_eq!(
            pretty_print(output_eof),
            pretty_print(build_tokens(
                "::core::compile_error!{\"unexpected end of input, expected an expression\"}"
            )),
            "Testing `FunctionToken` unsuccessful parsing: End of file"
        );

        Result::Ok(())
    }

    #[test]
    fn should_correctly_parse_function_operand_token() -> Result<(), String> {
        use quote::ToTokens;
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::OperandToken;

        let input = "| x | x + 1";
        let output;

        if let OperandToken::Function(op) =
            parse_test::<OperandToken>(build_tokens(input)).map_err(|error| error.to_string())?
        {
            output = op.value.to_token_stream().to_string();
        } else {
            panic!("Unexpected `OperandToken` variant");
        }

        assert_eq!(output, input, "Testing `OperandToken` successful parsing: `Function`");

        Result::Ok(())
    }

    #[test]
    fn should_correctly_parse_operation_operand_token() -> Result<(), String> {
        use quote::ToTokens;
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::{OperandToken, OperatorToken};

        let operand = "| x | x + 1";

        let get_input = |operator: &str| -> String { format!("{operand} {operator} {operand}") };

        let after = "<|";
        let before = "|>";

        let expected_after = (operand.to_owned(), OperatorToken::After, operand.to_owned());
        let expected_before = (operand.to_owned(), OperatorToken::Before, operand.to_owned());
        let input_after = get_input(after);
        let input_before = get_input(before);
        let output_after;
        let output_before;

        if let OperandToken::Operation(op) =
            parse_test::<OperandToken>(build_tokens(&input_after)).map_err(|error| error.to_string())?
        {
            if let (OperandToken::Function(lhs), OperandToken::Function(rhs)) = (*op.lhs, *op.rhs) {
                output_after = (
                    lhs.value.to_token_stream().to_string(),
                    op.operator,
                    rhs.value.to_token_stream().to_string(),
                );
            } else {
                panic!("Incorrect `OperationToken`")
            }
        } else {
            panic!("Unexpected `OperandToken` variant");
        }

        if let OperandToken::Operation(op) =
            parse_test::<OperandToken>(build_tokens(&input_before)).map_err(|error| error.to_string())?
        {
            if let (OperandToken::Function(lhs), OperandToken::Function(rhs)) = (*op.lhs, *op.rhs) {
                output_before = (
                    lhs.value.to_token_stream().to_string(),
                    op.operator,
                    rhs.value.to_token_stream().to_string(),
                );
            } else {
                panic!("Incorrect `OperationToken`")
            }
        } else {
            panic!("Unexpected `OperandToken` variant");
        }

        assert_eq!(
            output_after, expected_after,
            "Testing `OperandToken` successful parsing: `OperationToken`"
        );
        assert_eq!(
            output_before, expected_before,
            "Testing `OperandToken` successful parsing: `OperationToken`"
        );

        Result::Ok(())
    }

    #[test]
    fn should_correctly_parse_parenthesized_operand_token() -> Result<(), String> {
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::OperandToken;

        let input = "(| x | x + 1)";

        if let OperandToken::Parenthesized(_) =
            parse_test::<OperandToken>(build_tokens(input)).map_err(|error| error.to_string())?
        {
            Result::Ok(())
        } else {
            panic!("Unexpected `OperandToken` variant");
        }
    }

    #[test]
    fn should_correctly_parse_insert_token() -> Result<(), String> {
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::InsertToken;

        let expected = InsertToken {};
        let input = "->>";
        let output = parse_test::<InsertToken>(build_tokens(input)).map_err(|error| error.to_string())?;

        assert_eq!(output, expected, "Testing `InsertToken` successful parsing");

        Result::Ok(())
    }

    #[test]
    fn should_correctly_fail_parsing_insert_token() -> Result<(), String> {
        use timrs_macro_utils::{
            error::unexpected_token_message,
            test::{build_tokens, parse_test, pretty_print},
        };

        use super::InsertToken;

        let expected_token = "`Insert (->>)`";
        let got_token = "INVALID_INPUT";

        let expected = format!(
            "::core::compile_error!{{\"{}\"}}",
            unexpected_token_message(expected_token, got_token)
        );
        let input = got_token;
        let output = parse_test::<InsertToken>(build_tokens(input)).unwrap_err();

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(&expected)),
            "Testing `InsertToken` unsuccessful parsing"
        );

        Result::Ok(())
    }

    #[test]
    fn should_have_the_correct_lookahead_token_size_for_insert_token() {
        use timrs_macro_utils::parse::LookaheadToken;

        use super::InsertToken;

        let expected = 3_usize;

        assert_eq!(InsertToken::SIZE, expected)
    }

    #[test]
    fn should_correctly_parse_input_token() -> Result<(), String> {
        use quote::ToTokens;
        use syn::{
            parse::{Parse, ParseStream},
            Result as SynResult,
        };
        use timrs_macro_utils::test::{build_tokens, parse_test};

        use super::{InputToken, InsertToken};

        #[derive(Debug, PartialEq)]
        struct Test {
            input_token: String,
            insert_token: InsertToken,
        }

        impl Parse for Test {
            fn parse(input: ParseStream) -> SynResult<Self> {
                SynResult::Ok(Self {
                    input_token: input
                        .parse::<InputToken>()
                        .map(|value| value.value.to_token_stream().to_string())?,
                    insert_token: input.parse::<InsertToken>()?,
                })
            }
        }

        fn get_input(expected: &str) -> String { format!("{expected} ->>") }

        let expected_block = "{ let x = 1 ; x }";
        let expected_identifier = "a";
        let expected_parenthesized = "(a)";
        let input_block = get_input(expected_block);
        let input_identifier = get_input(expected_identifier);
        let input_parenthesized = get_input(expected_parenthesized);
        let output_block = parse_test::<Test>(build_tokens(&input_block)).map_err(|error| error.to_string())?;
        let output_identifier =
            parse_test::<Test>(build_tokens(&input_identifier)).map_err(|error| error.to_string())?;
        let output_parenthesized =
            parse_test::<Test>(build_tokens(&input_parenthesized)).map_err(|error| error.to_string())?;

        assert_eq!(
            output_block.input_token, expected_block,
            "Testing `InputToken` successful parsing: Block"
        );
        assert_eq!(
            output_identifier.input_token, expected_identifier,
            "Testing `InputToken` successful parsing: Identifier"
        );
        assert_eq!(
            output_parenthesized.input_token, expected_parenthesized,
            "Testing `InputToken` successful parsing: Parenthesized"
        );

        Result::Ok(())
    }

    #[test]
    fn should_correctly_fail_parsing_input_token() -> Result<(), String> {
        use quote::ToTokens;
        use timrs_macro_utils::{
            error::unexpected_end_of_stream_message,
            test::{build_tokens, parse_test, pretty_print},
        };

        use super::InputToken;

        let input_not_expression = "+ ->>";
        let output_not_expression = parse_test::<InputToken>(build_tokens(input_not_expression))
            .map(|value| value.value.to_token_stream().to_string())
            .unwrap_err();

        assert_eq!(
            pretty_print(output_not_expression),
            pretty_print(build_tokens("::core::compile_error!{\"expected an expression\"}")),
            "Testing `InputToken` unsuccessful parsing: Not an expression"
        );

        let input_eof = "";
        let output_eof = parse_test::<InputToken>(build_tokens(input_eof))
            .map(|value| value.value.to_token_stream().to_string())
            .unwrap_err();

        assert_eq!(
            pretty_print(output_eof),
            pretty_print(build_tokens(&format!(
                "::core::compile_error!{{\"{}\"}}",
                unexpected_end_of_stream_message()
            ))),
            "Testing `InputToken` unsuccessful parsing: End of file"
        );

        Result::Ok(())
    }

    #[test]
    fn should_generate_correct_pipe_resolution_for_after() -> Result<(), String> {
        use timrs_macro_utils::test::build_tokens;

        use super::impl_pipe;

        fn get_simple_pipe(input: &str, operation: &str) -> String { format!("{input} ->> {operation}") }
        fn get_parenthesized_pipe(input: &str, operation: &str) -> String {
            get_simple_pipe(input, &format!("({operation})"))
        }

        let input = "1";

        let trivial_operation = "g <| f";
        let closure_operation = "|x| x.to_string() <| |x| x + 1";
        let associative_operation = "h <| g <| f";
        let compound_operation = "(i <| h) <| (g <| f)";

        let expected_trivial = "(g) ((f) (1))";
        let expected_closure = "(| x | x . to_string ()) ((| x | x + 1) (1))";
        let expected_associative = "(h) ((g) ((f) (1)))";
        let expected_compound = "(i) ((h) ((g) ((f) (1))))";
        let input_trivial_operation = get_simple_pipe(input, trivial_operation);
        let input_trivial_parenthesized_operation = get_parenthesized_pipe(input, trivial_operation);
        let input_closure_operation = get_simple_pipe(input, closure_operation);
        let input_closure_parenthesized_operation = get_parenthesized_pipe(input, closure_operation);
        let input_associative_operation = get_simple_pipe(input, associative_operation);
        let input_associative_parenthesized_operation = get_parenthesized_pipe(input, associative_operation);
        let input_compound_operation = get_simple_pipe(input, compound_operation);
        let input_compound_parenthesized_operation = get_parenthesized_pipe(input, compound_operation);

        assert_eq!(
            impl_pipe(build_tokens(&input_trivial_operation)).to_string(),
            build_tokens(expected_trivial).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Trivial Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_trivial_parenthesized_operation)).to_string(),
            build_tokens(expected_trivial).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Trivial Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_closure_operation)).to_string(),
            build_tokens(expected_closure).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Closure Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_closure_parenthesized_operation)).to_string(),
            build_tokens(expected_closure).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Closure Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_associative_operation)).to_string(),
            build_tokens(expected_associative).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Associative Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_associative_parenthesized_operation)).to_string(),
            build_tokens(expected_associative).to_string(),
            "Testing `impl_pipe` `after` operation resolution: Associative Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_compound_operation)).to_string(),
            build_tokens(expected_compound).to_string(),
            "Testing `impl_pipe` `after` operation resolution: compound Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_compound_parenthesized_operation)).to_string(),
            build_tokens(expected_compound).to_string(),
            "Testing `impl_pipe` `after` operation resolution: compound Parenthesized Operation"
        );

        Result::Ok(())
    }

    #[test]
    fn should_generate_correct_pipe_resolution_for_before() -> Result<(), String> {
        use timrs_macro_utils::test::build_tokens;

        use super::impl_pipe;

        fn get_simple_pipe(input: &str, operation: &str) -> String { format!("{input} ->> {operation}") }
        fn get_parenthesized_pipe(input: &str, operation: &str) -> String {
            get_simple_pipe(input, &format!("({operation})"))
        }

        let input = "1";

        let trivial_operation = "f |> g";
        let closure_operation = "|x| x + 1 |> |x| x.to_string()";
        let associative_operation = "f |> g |> h";
        let compound_operation = "(f |> g) |> (h |> i)";

        let expected_trivial = "(g) ((f) (1))";
        let expected_closure = "(| x | x . to_string ()) ((| x | x + 1) (1))";
        let expected_associative = "(h) ((g) ((f) (1)))";
        let expected_compound = "(i) ((h) ((g) ((f) (1))))";
        let input_trivial_operation = get_simple_pipe(input, trivial_operation);
        let input_trivial_parenthesized_operation = get_parenthesized_pipe(input, trivial_operation);
        let input_closure_operation = get_simple_pipe(input, closure_operation);
        let input_closure_parenthesized_operation = get_parenthesized_pipe(input, closure_operation);
        let input_associative_operation = get_simple_pipe(input, associative_operation);
        let input_associative_parenthesized_operation = get_parenthesized_pipe(input, associative_operation);
        let input_compound_operation = get_simple_pipe(input, compound_operation);
        let input_compound_parenthesized_operation = get_parenthesized_pipe(input, compound_operation);

        assert_eq!(
            impl_pipe(build_tokens(&input_trivial_operation)).to_string(),
            build_tokens(expected_trivial).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Trivial Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_trivial_parenthesized_operation)).to_string(),
            build_tokens(expected_trivial).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Trivial Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_closure_operation)).to_string(),
            build_tokens(expected_closure).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Closure Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_closure_parenthesized_operation)).to_string(),
            build_tokens(expected_closure).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Closure Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_associative_operation)).to_string(),
            build_tokens(expected_associative).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Associative Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_associative_parenthesized_operation)).to_string(),
            build_tokens(expected_associative).to_string(),
            "Testing `impl_pipe` `before` operation resolution: Associative Parenthesized Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_compound_operation)).to_string(),
            build_tokens(expected_compound).to_string(),
            "Testing `impl_pipe` `before` operation resolution: compound Operation"
        );
        assert_eq!(
            impl_pipe(build_tokens(&input_compound_parenthesized_operation)).to_string(),
            build_tokens(expected_compound).to_string(),
            "Testing `impl_pipe` `before` operation resolution: compound Parenthesized Operation"
        );

        Result::Ok(())
    }
}
