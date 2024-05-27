use proc_macro2::TokenStream;
use syn::{buffer::Cursor, parse::Parse, parse2, Error as SynError, Result as SynResult};

use crate::error::unexpected_end_of_stream_message;

/// Accumulate [`proc_macro2::TokenTree`] into a [`proc_macro2::TokenStream`] while the given predicate returns `true` returning a tuple containing the resulting [`proc_macro2::TokenStream`] and a [`syn::buffer::Cursor`] positioned at the point where accumulation ended.
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use proc_macro2::TokenStream;
/// use syn::{
///     parse::{Parse, ParseStream},
///     parse2, Result as SynResult,
/// };
/// use timrs_macro_utils::parse::accumulate_while;
///
/// #[derive(Debug, PartialEq)]
/// struct Test { value: String }
///
/// impl Parse for Test {
///     fn parse(input: ParseStream) -> SynResult<Self> {
///         SynResult::Ok(Self {
///             value: input
///                 .step(|cursor| accumulate_while(*cursor, |current| !current.eof()))?
///                 .to_string(),
///         })
///     }
/// }
///
/// assert_eq!(
///     parse2::<Test>(
///         TokenStream::from_str("ARBITRARY_TOKEN").map_err(|error| error.to_string()).unwrap()
///     ).map_err(|error| error.to_string()).unwrap(),
///     Test { value: "ARBITRARY_TOKEN".to_owned() }
/// );
/// ```
///
/// # Errors
/// Returns the [`syn::Error`] if the cursor hits `eof` before the predicate returns `false`.
#[inline]
pub fn accumulate_while<F>(cursor: Cursor, mut predicate: F) -> SynResult<(TokenStream, Cursor)>
where
    F: FnMut(Cursor) -> bool,
{
    let mut current = cursor;
    let mut tokens = Vec::new();

    while predicate(current) {
        match current.token_tree() {
            Option::Some((tt, next)) => {
                tokens.push(tt);
                current = next;
            }
            Option::None => return SynResult::Err(SynError::new(current.span(), unexpected_end_of_stream_message())),
        }
    }

    SynResult::Ok((TokenStream::from_iter(tokens), current))
}

/// Trait defining a token that can be consumed by [`syn::parse::ParseBuffer::step`].
pub trait LookaheadToken: Parse {
    /// The size in number tokens for the [`Self`].
    const SIZE: usize;

    /// Looks ahead in the provided [`syn::buffer::Cursor`] for [`Self`] returning a tuple containing the parsed [`Self`] and a [`syn::buffer::Cursor`] positioned after the parsed token if it is found.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use proc_macro2::TokenStream;
    /// use syn::{
    ///     parse::{Parse, ParseStream},
    ///     parse2, Ident, Result as SynResult, Token,
    /// };
    /// use timrs_macro_utils::parse::LookaheadToken;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct TestToken;
    ///
    /// impl Parse for TestToken {
    ///     fn parse(input: ParseStream) -> SynResult<Self> {
    ///         input.parse::<Token![<]>()?;
    ///         input.parse::<Token![|]>()?;
    ///         input.parse::<Token![>]>()?;
    ///
    ///         SynResult::Ok(Self {})
    ///     }
    /// }
    ///
    /// impl LookaheadToken for TestToken { const SIZE: usize = 3; }
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Test { first: String, token: TestToken, second: String }
    ///
    /// impl Parse for Test {
    ///     fn parse(input: ParseStream) -> SynResult<Self> {
    ///         SynResult::Ok(Self {
    ///             first: input.parse::<Ident>()?.to_string(),
    ///             token: input.step(|cursor| TestToken::lookahead (*cursor))?,
    ///             second: input.parse::<Ident>()?.to_string(),
    ///         })
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     parse2::<Test>(
    ///         TokenStream::from_str("first <|> second").map_err(|error| error.to_string()).unwrap()
    ///     ).map_err(|error| error.to_string()).unwrap(),
    ///     Test { first: "first".to_owned(), token: TestToken, second: "second".to_owned() }
    /// );
    /// ```
    ///
    /// # Errors
    /// Returns the [`syn::Error`] for the attempted lookahead if the [`Self`] can't be parsed.
    #[allow(clippy::arithmetic_side_effects)]
    #[inline]
    fn lookahead(cursor: Cursor) -> SynResult<(Self, Cursor)> {
        let mut limit = 0;

        let (token_stream, current) = accumulate_while(cursor, |_| {
            let should_continue = limit < Self::SIZE;

            if should_continue {
                limit += 1;
            }

            should_continue
        })?;

        parse2::<Self>(token_stream).map(|value| (value, current))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_accumulate_into_a_token_stream_and_move_the_cursor() -> Result<(), String> {
        use crate::test::{build_tokens, parse_test};
        use syn::{
            parse::{Parse, ParseStream},
            Result as SynResult,
        };

        use super::accumulate_while;

        #[derive(Debug, PartialEq)]
        struct Test {
            value: String,
        }

        impl Parse for Test {
            fn parse(input: ParseStream) -> SynResult<Self> {
                SynResult::Ok(Self {
                    value: input
                        .step(|cursor| accumulate_while(*cursor, |current| !current.eof()))?
                        .to_string(),
                })
            }
        }

        let value = "ARBITRARY_VALUE";
        let expected = Test {
            value: value.to_owned(),
        };
        let input = value;
        let output = parse_test::<Test>(build_tokens(&input)).map_err(|error| error.to_string())?;

        assert_eq!(
            output, expected,
            "Testing `accumulate_while` parsing and cursor updates."
        );

        Result::Ok(())
    }

    #[test]
    fn should_fail_when_accumulation_ends_before_predicate_returns_false() -> Result<(), String> {
        use crate::error::unexpected_end_of_stream_message;
        use crate::test::{build_tokens, parse_test, pretty_print};
        use syn::{
            parse::{Parse, ParseStream},
            Result as SynResult,
        };

        use super::accumulate_while;

        #[derive(Debug, PartialEq)]
        struct Test {
            value: String,
        }

        impl Parse for Test {
            fn parse(input: ParseStream) -> SynResult<Self> {
                SynResult::Ok(Self {
                    value: input.step(|cursor| accumulate_while(*cursor, |_| true))?.to_string(),
                })
            }
        }

        let expected = format!("::core::compile_error!{{\"{}\"}}", unexpected_end_of_stream_message());
        let input = "ARBITRARY_VALUE";
        let output = parse_test::<Test>(build_tokens(&input)).unwrap_err();

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(&expected)),
            "Testing `accumulate_while` accumulation failures."
        );

        Result::Ok(())
    }

    #[test]
    fn should_parse_valid_lookahead_token_and_move_the_cursor() -> Result<(), String> {
        use crate::test::{build_tokens, parse_test};
        use syn::{
            parse::{Parse, ParseStream},
            Ident, Result as SynResult, Token,
        };

        use super::LookaheadToken;

        #[derive(Debug, PartialEq)]
        struct TestToken;

        impl Parse for TestToken {
            fn parse(input: ParseStream) -> SynResult<Self> {
                input.parse::<Token![<]>()?;
                input.parse::<Token![|]>()?;
                input.parse::<Token![>]>()?;

                SynResult::Ok(Self {})
            }
        }

        impl LookaheadToken for TestToken {
            const SIZE: usize = 3;
        }

        #[derive(Debug, PartialEq)]
        struct Test {
            first: String,
            token: TestToken,
            second: String,
        }

        impl Parse for Test {
            fn parse(input: ParseStream) -> SynResult<Self> {
                SynResult::Ok(Self {
                    first: input.parse::<Ident>()?.to_string(),
                    token: input.step(|cursor| TestToken::lookahead(*cursor))?,
                    second: input.parse::<Ident>()?.to_string(),
                })
            }
        }

        let first = "first";
        let second = "second";
        let expected = Test {
            first: first.to_owned(),
            token: TestToken,
            second: second.to_owned(),
        };
        let input = format!("{first} <|> {second}");
        let output = parse_test::<Test>(build_tokens(&input)).map_err(|error| error.to_string())?;

        assert_eq!(
            output, expected,
            "Testing `LookahedToken.lookahead` parsing and cursor updates."
        );

        Result::Ok(())
    }
}
