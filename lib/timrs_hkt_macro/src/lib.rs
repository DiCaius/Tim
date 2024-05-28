#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Error as SynError, Parse, ParseStream, Result as SynResult},
    parse2, DeriveInput, LitInt,
};

struct HktToken {
    arity: usize,
}

impl Parse for HktToken {
    fn parse(input: ParseStream) -> SynResult<Self> {
        input
            .parse()
            .and_then(|arity_literal: LitInt| arity_literal.base10_parse())
            .and_then(|arity: usize| {
                if arity >= 1 {
                    SynResult::Ok(Self { arity })
                } else {
                    SynResult::Err(SynError::new(
                        input.span(),
                        format!("Arity for a HKT must be of at least `1`: Got {arity}"),
                    ))
                }
            })
    }
}

fn impl_hkt(tokens: TokenStream2) -> TokenStream2 {
    parse2::<HktToken>(tokens).map_or_else(SynError::into_compile_error, |input| {
        let trait_name = format_ident!("HKT{}", input.arity);
        let mut trait_types = Vec::new();
        let mut trait_with_types = Vec::new();

        for index in 1..=input.arity {
            trait_types.push(format_ident!("T{}", index));
            trait_with_types.push(format_ident!("_T{}", index));
        }

        quote! {
            pub trait #trait_name {
                #(type #trait_types;)*
                type With<#(#trait_with_types),*>: #trait_name<#(#trait_types = #trait_with_types),*>
                    + #trait_name<With<#(Self::#trait_types),*> = Self>
                    + #trait_name<With<#(#trait_with_types),*> = Self::With<#(#trait_with_types),*>>;
            }
        }
    })
}

fn impl_hkt_derive(tokens: TokenStream2) -> TokenStream2 {
    parse2::<DeriveInput>(tokens).map_or_else(SynError::into_compile_error, |ast| {
        let mut type_params = Vec::new();

        for param in &ast.generics.params {
            if let syn::GenericParam::Type(ref type_param) = *param {
                type_params.push(type_param);
            }
        }

        let name = &ast.ident;
        let type_param_names = type_params.iter().map(|type_param| &type_param.ident);
        let type_param_bounds = type_params.iter().map(|type_param| &type_param.bounds);
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        let trait_name = format_ident!("HKT{}", type_params.len());
        let mut trait_types = Vec::new();
        let mut trait_with_types = Vec::new();

        for index in 1..=type_params.len() {
            trait_types.push(format_ident!("T{}", index));
            trait_with_types.push(format_ident!("_T{}", index));
        }

        quote! {
            impl #impl_generics #trait_name for #name #ty_generics #where_clause {
                #(type #trait_types = #type_param_names;)*
                type With<#(#trait_with_types: #type_param_bounds),*> = #name<#(#trait_with_types),*>;
            }
        }
    })
}

/// Macro for defining a Higher-Kinded Type of a specific arity.
///
/// This macro will generate a public trait with the format:
/// ```code
/// pub trait HKT{N} {
///     type T1;
///     ...;
///     type T{N};
///     type With<_T1, ..., T{N}>: ...
/// }
/// ```
/// Where `{N}` is the arity of your Higher-Kinded Type.
///
/// The current type being stored in the generic parameter in the position `N` can be accessed through the associated type `Self::T{N}` and a new instance of a Higher-Kinded Type with types `X` to `X{N}` can be instantiated via the generic associated type `Self::With<X, ..., X{N}>`.
///
/// # Examples
/// ```
/// use timrs_hkt_macro::{hkt, HKT};
///
/// /* The parameter indicates the arity of the Higher-Kinded Type trait to be generated */
/// hkt!(1);
///
/// trait Functor: HKT1 {
///     fn map<B, F>(self, f: F) -> Self::With<B>
///     where
///         F: FnMut(Self::T1) -> B;
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum Example1<A> {
///     First(A),
/// }
///
/// impl<A> HKT1 for Example1<A> {
///     type T1 = A;
///     type With<_T1> = Example1<_T1>;
/// }
///
/// impl<A> Functor for Example1<A> {
///     fn map<B, F>(self, mut f: F) -> Self::With<B>
///     where
///         F: FnMut(Self::T1) -> B,
///     {
///         match self {
///             Self::First(a) => Example1::First(f(a)),
///         }
///     }
/// }
///
/// assert_eq!(
///     Example1::First("The magic number is 42".to_owned()),
///     Example1::First(42).map(|x| "The magic number is ".to_owned() + &x.to_string())
/// );
/// ```
#[proc_macro]
pub fn hkt(tokens: TokenStream) -> TokenStream { impl_hkt(tokens.into()).into() }

/// Macro for deriving implementations of [`crate::hkt!`].
///
/// Arity of the Higher-Kinded Type is automatically defined based on the implementing type arity.
///
/// # Examples
/// ```
/// use timrs_hkt_macro::{hkt, HKT};
///
/// /* The parameter indicates the arity of the Higher-Kinded Type trait to be generated */
/// hkt!(1);
///
/// trait Functor: HKT1 {
///     fn map<B, F>(self, f: F) -> Self::With<B>
///     where
///         F: FnMut(Self::T1) -> B;
/// }
///
/// #[derive(HKT, Debug, PartialEq)]
/// enum Example1<A> {
///     First(A),
/// }
///
/// impl<A> Functor for Example1<A> {
///     fn map<B, F>(self, mut f: F) -> Self::With<B>
///     where
///         F: FnMut(Self::T1) -> B,
///     {
///         match self {
///             Self::First(a) => Example1::First(f(a)),
///         }
///     }
/// }
///
/// assert_eq!(
///     Example1::First("The magic number is 42".to_owned()),
///     Example1::First(42).map(|x| "The magic number is ".to_owned() + &x.to_string())
/// );
/// ```
#[proc_macro_derive(HKT)]
pub fn hkt_derive(tokens: TokenStream) -> TokenStream { impl_hkt_derive(tokens.into()).into() }

#[cfg(test)]
mod tests {
    #[test]
    fn hkt_should_create_trait_for_zero_arity() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt;

        let expected = "::core::compile_error! {\"Arity for a HKT must be of at least `1`: Got 0\"}";
        let input = "0";
        let output = impl_hkt(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing zero arity Higher-Kinded Types"
        );
    }

    #[test]
    fn hkt_should_create_trait_for_unary() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt;

        let expected = r###"
            pub trait HKT1 {
                type T1;
                type With<_T1>:
                    HKT1<T1 = _T1>
                    + HKT1<With<Self::T1> = Self>
                    + HKT1<With<_T1> = Self::With<_T1>>;
            }
        "###;
        let input = "1";
        let output = impl_hkt(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing unary Higher-Kinded Types"
        );
    }

    #[test]
    fn hkt_should_create_trait_for_higher_arity() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt;

        let expected = r###"
            pub trait HKT2 {
                type T1;
                type T2;
                type With<_T1, _T2>:
                    HKT2<T1 = _T1, T2 = _T2>
                    + HKT2<With<Self::T1, Self::T2> = Self>
                    + HKT2<With<_T1, _T2> = Self::With<_T1, _T2>>;
            }
        "###;
        let input = "2";
        let output = impl_hkt(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing higher arity Higher-Kinded Types"
        );
    }

    #[test]
    fn hkt_should_fail_if_arity_is_invalid() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt;

        let output_not_provided = impl_hkt(build_tokens(""));
        let output_float = impl_hkt(build_tokens("2.2"));
        let output_ident = impl_hkt(build_tokens("A"));
        let output_string = impl_hkt(build_tokens("\"A\""));

        assert_eq!(
            pretty_print(output_not_provided),
            pretty_print(build_tokens(
                "::core::compile_error! {\"unexpected end of input, expected integer literal\"}"
            )),
            "Testing failures for Higher-Kinded Types with no arity provided"
        );

        assert_eq!(
            pretty_print(output_float),
            pretty_print(build_tokens("::core::compile_error! {\"expected integer literal\"}")),
            "Testing failures for Higher-Kinded Types with improper arity: Float"
        );

        assert_eq!(
            pretty_print(output_ident),
            pretty_print(build_tokens("::core::compile_error! {\"expected integer literal\"}")),
            "Testing failures for Higher-Kinded Types with improper arity: Ident"
        );
        assert_eq!(
            pretty_print(output_string),
            pretty_print(build_tokens("::core::compile_error! {\"expected integer literal\"}")),
            "Testing failures for Higher-Kinded Types with improper arity: String"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_unary() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A> HKT1 for HKT<A> {
                type T1 = A;
                type With<_T1> = HKT<_T1>;
            }
        "###;
        let input = r###"
            enum HKT<A> {
                T1(A)
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing unary Higher-Kinded Types derivation"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_unary_with_bounds() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A: Sized> HKT1 for HKT<A> {
                type T1 = A;
                type With<_T1: Sized> = HKT<_T1>;
            }
        "###;
        let input = r###"
            enum HKT<A: Sized> {
                T1(A)
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing unary Higher-Kinded Types derivation with bounds"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_unary_with_where_clause() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A> HKT1 for HKT<A> where A: Sized {
                type T1 = A;
                type With<_T1> = HKT<_T1>;
            }
        "###;
        let input = r###"
            enum HKT<A> where A: Sized {
                T1(A)
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing unary Higher-Kinded Types derivation with where clause"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_unary_with_bounds_and_where_clause() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A: Copy> HKT1 for HKT<A> where A: Sized {
                type T1 = A;
                type With<_T1: Copy> = HKT<_T1>;
            }
        "###;
        let input = r###"
            enum HKT<A: Copy> where A: Sized {
                T1(A)
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing unary Higher-Kinded Types derivation with bounds and where clause"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_higher_arity() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A, B> HKT2 for HKT<A, B> {
                type T1 = A;
                type T2 = B;
                type With<_T1, _T2> = HKT<_T1, _T2>;
            }
        "###;
        let input = r###"
            enum HKT<A, B> {
                T1(A),
                T2(B),
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing higher arity Higher-Kinded Types derivation"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_higher_arity_with_bounds() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A: Sized, B: Sized> HKT2 for HKT<A, B> {
                type T1 = A;
                type T2 = B;
                type With<_T1: Sized, _T2: Sized> = HKT<_T1, _T2>;
            }
        "###;
        let input = r###"
            enum HKT<A: Sized, B: Sized> {
                T1(A),
                T2(B),
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing higher arity Higher-Kinded Types derivation with bounds"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_higher_arity_with_where_clause() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A, B> HKT2 for HKT<A, B> where A: Sized, B: Sized {
                type T1 = A;
                type T2 = B;
                type With<_T1, _T2> = HKT<_T1, _T2>;
            }
        "###;
        let input = r###"
            enum HKT<A, B> where A: Sized, B: Sized {
                T1(A),
                T2(B),
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing higher arity Higher-Kinded Types derivation with where clause"
        );
    }

    #[test]
    fn hkt_derive_should_implement_hkt_trait_for_higher_arity_with_bounds_and_where_clause() {
        use timrs_macro_utils::test::{build_tokens, pretty_print};

        use super::impl_hkt_derive;

        let expected = r###"
            impl<A: Copy, B: Copy> HKT2 for HKT<A, B> where A: Sized, B: Sized {
                type T1 = A;
                type T2 = B;
                type With<_T1: Copy, _T2: Copy> = HKT<_T1, _T2>;
            }
        "###;
        let input = r###"
            enum HKT<A: Copy, B: Copy> where A: Sized, B: Sized {
                T1(A),
                T2(B),
            }
        "###;
        let output = impl_hkt_derive(build_tokens(input));

        assert_eq!(
            pretty_print(output),
            pretty_print(build_tokens(expected)),
            "Testing higher arity Higher-Kinded Types derivation with bounds and where clause"
        );
    }
}
