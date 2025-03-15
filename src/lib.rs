//! This crate contains a little macro to generate a lazy
//! [`Regex`](../regex/struct.Regex.html) and remove some boilerplate when
//! compiling regex expressions.
//!
//! # Usage
//!
//! Generally you want to avoid compiling a regex multiple times, by using a
//! static variable to store the compiled regex. The macro does this using
//! [`std::sync::LazyLock`].
//!
//! ```rust
//! use regex_macro::regex;
//!
//! let re = regex!("[0-9a-f]+");
//! assert!(re.is_match("1234deadbeef"));
//! ```
//!
//! Which is basically equivalent to the following.
//! ```rust
//! use std::sync::LazyLock;
//! use regex::Regex;
//!
//! static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new("[0-9a-f]+").unwrap());
//! assert!(RE.is_match("1234deadbeef"));
//! ```

#[doc(hidden)]
pub type Regex = regex::Regex;
#[doc(hidden)]
pub type Lazy = std::sync::LazyLock<Regex>;

/// Generate a static regex.
#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::Lazy = $crate::Lazy::new(|| $crate::Regex::new($re).unwrap());
        $crate::Lazy::force(&RE)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let hex = regex!("[0-9a-f]+");
        assert!(hex.is_match("1234deadbeef"));
    }
}
