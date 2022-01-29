# regex-macro

[![Crates.io Version](https://img.shields.io/crates/v/regex-macro.svg)](https://crates.io/crates/regex-macro)
[![Docs.rs Latest](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/regex-macro)
[![Build Status](https://img.shields.io/github/workflow/status/rossmacarthur/regex-macro/build/trunk)](https://github.com/rossmacarthur/regex-macro/actions?query=workflow%3Abuild)

This crate contains a little macro to generate a lazy
[`Regex`](https://docs.rs/regex/latest/regex/struct.Regex.html) and remove some
boilerplate when compiling regex expressions.

## Usage

Generally you want to avoid compiling a regex multiple times. The `regex`
crate suggests using `lazy_static` for this but you can also use `once_cell`
which is what this crate uses. For example:

```rust
use regex_macro::regex;

let re = regex!("[0-9a-f]+");
assert!(re.is_match("1234deadbeef"));
```

Which is equivalent to the following.

```rust
use once_cell::sync::Lazy;
use regex::Regex;

let re = {
  static RE: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9a-f]+").unwrap());
  &*RE
};
assert!(re.is_match("1234deadbeef"));
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
