/// Generate a static regex.
#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: ::once_cell::sync::OnceCell<regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| ::regex::Regex::new($re).unwrap())
    }};
}

/// Try generate a static regex.
#[macro_export]
macro_rules! try_regex {
    ($re:expr $(,)?) => {{
        static RE: ::once_cell::sync::OnceCell<regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_try_init(|| ::regex::Regex::new($re))
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
