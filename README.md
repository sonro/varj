# varj ![Build](https://github.com/sonro/varj/workflows/Rust/badge.svg) [![Crates.io](https://img.shields.io/crates/v/varj.svg)](https://crates.io/crates/varj/) [![Documentation](https://docs.rs/varj/badge.svg)](https://docs.rs/varj/)

A string interpolation utility to replace
[Mustache](https://mustache.github.io/) like placeholders with stored variables.

- Works as an extremely lightweight template library
- Does not require template compilation
- Simply replaces `{{ key }}` with `value`
- Whitespace surrounding the key is ignored: `{{key}}` and `{{ key }}` are equal.

Interact with this utility via `VarjMap`.

## Examples

Basic usage:

```rust
let mut vars = varj::VarjMap::new();
vars.insert("key", "value");

let expected = "value";
let actual = vars.parse("{{ key }}")?;

assert_eq!(expected, actual);
```

With a json string:

```rust
let mut variables = varj::VarjMap::new();
variables.insert("name", "Christopher");
variables.insert("age", "30");

let json = r#"{
"name" = "{{ name }}",
"age" = {{ age }}
}"#;

let expected = r#"{
"name" = "Christopher",
"age" = 30
}"#;

let actual = variables.parse(json)?;

assert_eq!(expected, actual);
```

## License

Varj is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
