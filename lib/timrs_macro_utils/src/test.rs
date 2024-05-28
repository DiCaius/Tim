use core::str::FromStr;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, Parser},
    parse2, Error as SynError, File,
};

/// Converts a `String` into a [`proc_macro2::TokenStream`].
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use timrs_macro_utils::test::build_tokens;
///
/// assert_eq!(
///     build_tokens("a").to_string(),
///     "a"
/// );
/// ```
///
/// # Panics
/// Will panic if the provided input can't be converted into a valid [`proc_macro2::TokenStream`].
#[allow(clippy::unwrap_used)]
#[inline]
pub fn build_tokens(input: &str) -> TokenStream { TokenStream::from_str(input).unwrap() }

/// Pretty prints a [`proc_macro2::TokenStream`] to a `String`.
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use proc_macro2::TokenStream;
/// use syn::{Ident};
/// use timrs_macro_utils::test::{build_tokens, parse_test};
///
/// assert_eq!(
///     parse_test::<Ident>(build_tokens("a")).map(|value| value.to_string()).unwrap(),
///     "a"
/// );
/// ```
///
/// # Errors
/// Will return a [`proc_macro2::TokenStream`] containing a compiler error in case parsing fails.
#[allow(clippy::unwrap_used)]
#[inline]
pub fn parse_test<T: Parse>(tokens: TokenStream) -> Result<T, TokenStream> {
    parse2::<T>(tokens).map_err(SynError::into_compile_error)
}

/// Pretty prints a [`proc_macro2::TokenStream`] to a `String`.
///
/// # Examples
/// ```
/// use proc_macro2::TokenStream;
/// use timrs_macro_utils::test::{build_tokens, pretty_print};
///
/// assert_eq!(
///     pretty_print(build_tokens("")),
///     ""
/// );
/// ```
///
/// # Panics
/// Will panic if an invalid [`proc_macro2::TokenStream`] is provided.
#[allow(clippy::unwrap_used)]
#[inline]
pub fn pretty_print(tokens: TokenStream) -> String { unparse(&File::parse.parse2(tokens).unwrap()) }
