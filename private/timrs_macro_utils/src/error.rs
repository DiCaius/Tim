/// Generate a message for an `Unexpected Token` error.
///
/// # Examples
/// ```
/// use timrs_macro_utils::error::{unexpected_end_of_stream_message};
///
/// assert_eq!(
///     unexpected_end_of_stream_message(),
///     "Unexpected End of Stream"
/// );
/// ```
#[inline]
pub fn unexpected_end_of_stream_message() -> String { "Unexpected End of Stream".to_owned() }

/// Generate a message for an `Unexpected Token` error.
///
/// # Examples
/// ```
/// use timrs_macro_utils::error::{unexpected_token_message};
///
/// assert_eq!(
///     unexpected_token_message("`A`", "`B`"),
///     "Unexpected Token: Expected `A`, Got `B`"
/// );
/// ```
#[inline]
pub fn unexpected_token_message(expected: &str, got: &str) -> String {
    format!("Unexpected Token: Expected {expected}, Got {got}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_generate_correct_unexpected_end_of_stream_message() {
        use super::unexpected_end_of_stream_message;

        assert_eq!(
            unexpected_end_of_stream_message(),
            "Unexpected End of Stream",
            "Testing `unexpected_end_of_stream_message` error message formatting.",
        )
    }

    #[test]
    fn should_generate_correct_unexpected_token_message() {
        use super::unexpected_token_message;

        let expected = "`A`";
        let got = "`B`";

        assert_eq!(
            unexpected_token_message(expected, got),
            format!("Unexpected Token: Expected {expected}, Got {got}"),
            "Testing `unexpected_token_message` error message formatting.",
        )
    }
}
