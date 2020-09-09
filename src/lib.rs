//! This crate contains a little macro to generate a lazy
//! [`Regex`](../regex/struct.Regex.html) and remove some boilerplate when
//! compiling regex expressions.

#[doc(hidden)]
pub type Regex = regex::Regex;
#[doc(hidden)]
pub type LazyRegex = once_cell::sync::OnceCell<Regex>;

/// Generate a static regex.
#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::LazyRegex = $crate::LazyRegex::new();
        RE.get_or_init(|| $crate::Regex::new($re).unwrap())
    }};
}

/// Try generate a static regex.
#[macro_export]
macro_rules! try_regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::LazyRegex = $crate::LazyRegex::new();
        RE.get_or_try_init(|| $crate::Regex::new($re))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let hex = regex!("[0-9a-f]+");
        assert!(hex.is_match("deadbeef"));
    }

    #[test]
    fn try_regex() -> Result<(), regex::Error> {
        let hex = try_regex!("[0-9a-f]+")?;
        assert!(hex.is_match("deadbeef"));
        Ok(())
    }
}
