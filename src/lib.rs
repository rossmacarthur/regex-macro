//! This crate contains a little macro to generate a lazy
//! [`Regex`](../regex/struct.Regex.html) and remove some boilerplate when
//! compiling regex expressions.
//!
//! # Usage
//!
//! Since it is an anti-pattern to compile the same regular expression in a loop
//! this means for most regex expressions you need to store them in a
//! [`LazyLock`] in a static. But this can get a bit tiresome with many regex
//! expressions. This crate provides a [`regex!`] macro that will store the
//! compiled regex in a global static and return a reference to it.
//!
//! [`LazyLock`]: std::sync::LazyLock
//!
//! ### Before
//!
//! ```
//! use std::sync::LazyLock;
//! use regex::Regex;
//!
//! // only compiled once, by storing in a static
//! static HEX_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
//!    Regex::new("[0-9a-f]+").expect("invalid regex")
//! });
//!
//! # let my_iter = vec!["deadbeaf", "1234"];
//! for item in my_iter {
//!     if HEX_PATTERN.is_match(item) {
//!        // frobnicate
//!     }
//! }
//! ```
//!
//! ### After
//!
//! ```
//! use regex_macro::regex;
//!
//! # let my_iter = vec!["deadbeaf", "1234"];
//! for item in my_iter {
//!     // this is still only compiled once!
//!     if regex!("[0-9a-f]+").is_match(item) {
//!        // frobnicate
//!     }
//! }
//! ```
//!
//! Isn't that much nicer?
//!

use std::sync::LazyLock;

#[doc(hidden)]
pub use regex::Regex;

/// Convenient type alias for a lazy regex.
pub type LazyRegex = LazyLock<Regex>;

/// Returns a lazy regex.
///
/// # Examples
///
/// ```
/// use regex_macro::{LazyRegex, lazy_regex};
///
/// static RE: LazyRegex = lazy_regex!("[0-9a-f]+");
/// ```
#[macro_export]
macro_rules! lazy_regex {
    ($re:expr $(,)?) => {{
        $crate::LazyRegex::new(|| $crate::Regex::new($re).expect("invalid regex"))
    }};
}

/// Creates a static regex and returns a reference to it.
///
/// See the [crate level documentation][crate] for more information.
#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::LazyRegex = $crate::lazy_regex!($re);
        &RE
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
