//! This crate contains a little macro to generate a lazy
//! [`Regex`](../regex/struct.Regex.html) and remove some boilerplate when
//! compiling regex expressions.
//!
//! # Usage
//!
//! Generally you want to avoid compiling a regex multiple times. The `regex`
//! crate suggests using `lazy_static` for this but you can also use `once_cell`
//! which is what this crate uses. For example:
//!
//! ```rust
//! use regex_macro::regex;
//!
//! let re = regex!("[0-9a-f]+");
//! assert!(re.is_match("1234deadbeef"));
//! ```
//!
//! Which is equivalent to the following.
//! ```rust
//! use once_cell::sync::Lazy;
//! use regex::Regex;
//!
//! static RE: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9a-f]+").unwrap());
//! assert!(RE.is_match("1234deadbeef"));
//! ```

#[doc(hidden)]
pub type Regex = regex::Regex;
#[doc(hidden)]
pub type Lazy = once_cell::sync::Lazy<Regex>;

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
