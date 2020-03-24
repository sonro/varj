# varj ![Build](https://github.com/sonro/varj/workflows/Rust/badge.svg) [![Crates.io](https://img.shields.io/crates/v/varj.svg)](https://crates.io/crates/varj/) [![Documentation](https://docs.rs/varj/badge.svg)](https://docs.rs/varj/)

A string interpolation utility to replace
[Mustache](https://mustache.github.io/) like placeholders with stored variables.

-   Works as an extremely lightweight template library
-   Does not require template compilation
-   Simply replaces `{{ key }}` with `value`

Interact with this utility via the `parse` function

## Examples

Basic usage:

```rust
use std::collections::HashMap;

let mut vars = HashMap::new();
vars.insert(
    "key".to_owned(),
    "value".to_owned()
);

assert_eq!(
    "value",
    varj::parse("{{ key }}", &vars)?
);
```

With a json string:

```rust
let mut variables = varj::VarjMap::new();
variables.insert("name".to_owned(), "Christopher".to_owned());
variables.insert("age".to_owned(), "30".to_owned());

let json = r#"{
    "name" = "{{ name }}",
    "age" = {{ age }}
}"#;

let expected = r#"{
    "name" = "Christopher",
    "age" = 30
}"#;

let actual = varj::parse(json, &variables)?;

assert_eq!(expected, actual);
```

## License

Varj is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
