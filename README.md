# varj

[![Crates.io](https://img.shields.io/crates/v/varj.svg)](https://crates.io/crates/varj)
[![msrv
1.56.1](https://img.shields.io/badge/msrv-1.56.1-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.56.1)
[![tests](https://img.shields.io/github/actions/workflow/status/sonro/varj/tests.yml?label=tests&logo=github)](https://github.com/sonro/varj/actions/workflows/tests.yml)
[![Documentation](https://img.shields.io/docsrs/varj?logo=docs.rs)](https://docs.rs/varj/)
[![license](https://img.shields.io/crates/l/varj.svg)](#license)

A string interpolation utility to replace
[Mustache](https://mustache.github.io/) like placeholders with stored variables.

- Works as an extremely lightweight template library
- Does not require template compilation
- Simply replaces `{{ key }}` with `value`
- Whitespace surrounding the key is ignored: `{{key}}` and `{{ key }}` are equal.

Interact with this utility via
[`VarjMap`](https://docs.rs/varj/latest/varj/struct.VarjMap.html).

## Examples

Basic usage:

```rust
let mut map = varj::VarjMap::new();
map.insert("key", "value");

let expected = "value";
let actual = map.render("{{ key }}")?;

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

let actual = variables.render(json)?;

assert_eq!(expected, actual);
```

`VarjMap` implements `From<HashMap>` and can be converted back to one when
needed.  This is useful if you want to build a `VarjMap` from an iterator,
or iterate over one. See [example](./examples/conversion.rs).

## MSRV Policy

The minimum supported Rust version is currently
[1.56.1](https://github.com/rust-lang/rust/releases/tag/1.56.1).

varj supports the latest 8 stable releases of Rust - approximately 1 year.
Increasing MSRV is *not* considered a semver-breaking change.

## Contributing

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features,
  ...)

**Note**: Before you take the time to open a pull request, please open an issue
first.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

varj is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
