# regex-macro

This crate contains a little macro to generate a lazy
[`Regex`](https://docs.rs/regex/latest/regex/struct.Regex.html) and remove some
boilerplate when compiling regex expressions.

## Usage

The following panics when given a bad regex.

```rust
use regex_macro::regex;

let re = regex!("[0-9a-f]+");
```

The following errors when given a bad regex.

```rust
use regex_macro::try_regex;

let re = try_regex!("[0-9a-f]+").expect("bad regex");

// or propagate

let re = try_regex!("[0-9a-f]+")?;
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
