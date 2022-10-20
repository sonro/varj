//! A string interpolation utility to replace
//! [Mustache](https://mustache.github.io/) like placeholders with stored variables.
//!
//!  - Works as an extremely lightweight template library
//!  - Does not require template compilation
//!  - Simply replaces `{{ key }}` with `value`
//!  - Whitespace surrounding the key is ignored: `{{key}}` and `{{ key }}` are equal.
//!
//! Interact with this utility via [`VarjMap`]
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let mut vars = varj::VarjMap::new();
//! vars.insert("key", "value");
//!
//! assert_eq!(
//!     "value",
//!     vars.parse("{{ key }}")?
//! );
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! With a json string:
//!
//! ```rust
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let mut variables = varj::VarjMap::new();
//! variables.insert("name", "Christopher");
//! variables.insert("age", "30");
//!
//! let json = r#"{
//!     "name" = "{{ name }}",
//!     "age" = {{ age }}
//! }"#;
//!
//! let expected = r#"{
//!     "name" = "Christopher",
//!     "age" = 30
//! }"#;
//!
//! let actual = variables.parse(json)?;
//!
//! assert_eq!(expected, actual);
//! #
//! #     Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::fmt;

/// A map of variables to replace placeholders in a string.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VarjMap {
    map: HashMap<String, String>,
}

impl VarjMap {
    /// Create an empty `VarjMap`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `VarjMap` with the specified capacity.
    ///
    /// The hash map will be able to hold at least `capacity` elements without
    /// reallocating. If `capacity` is 0, the hash map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Insert a key value pair into the `VarjMap`.
    ///
    /// Use any type so long as it can be converted into a string.
    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map.insert(key.into(), value.into());
    }

    /// Get a value from the `VarjMap` by key.
    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&str> {
        self.map.get(key.as_ref()).map(|s| s.as_str())
    }

    /// Parse the input for VarjBlocks `{{ key }}` and replace with value
    ///
    /// If no VarjBlocks are present in the input string, it is merely cloned.
    ///
    /// Whitespace surrounding the key is ignored:
    /// `{{key}}` and `{{ key }}` are equal.
    ///
    /// # Errors
    ///
    /// Will return an error if the input string contains a key that is not
    /// provided.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut map = varj::VarjMap::new();
    ///
    /// // add variables to VarjMap
    /// let key = "name";
    /// let value = "Christopher";
    /// map.insert(key, value);
    ///
    /// // string input to parse
    /// let input = "name: {{name}}";
    ///
    /// // test result
    /// let expected = "name: Christopher";
    /// let actual = map.parse(input)?;
    ///
    /// assert_eq!(expected, actual);
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn parse(&self, input: &str) -> Result<String, Error> {
        let blocks = generate_blocks(input);

        let mut output = String::with_capacity(input.len() + 32);
        let mut idx = 0;

        for block in blocks {
            // copy input until block
            output.push_str(&input[idx..block.start]);
            idx = block.start;

            // copy variable_value
            if let Some(value) = block.value_from_map(self) {
                output.push_str(value);
            } else {
                return Err(Error::from(block));
            }

            // update idx to end of block
            idx += block.len;
        }

        // copy remaining input
        output.push_str(&input[idx..input.len()]);

        Ok(output)
    }
}

impl From<HashMap<String, String>> for VarjMap {
    fn from(map: HashMap<String, String>) -> Self {
        VarjMap { map }
    }
}

impl From<VarjMap> for HashMap<String, String> {
    fn from(map: VarjMap) -> Self {
        map.map
    }
}

/// Unknown key in input string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    key: String,
    line: usize,
    col: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{} unknown variable '{}'",
            self.line, self.col, self.key
        )
    }
}

impl std::error::Error for Error {}

impl From<Block<'_>> for Error {
    fn from(block: Block) -> Error {
        Error {
            key: block.variable_key.to_owned(),
            line: block.line,
            col: block.col,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Block<'a> {
    start: usize,
    len: usize,
    line: usize,
    col: usize,
    variable_key: &'a str,
}

impl<'a> Block<'a> {
    fn value_from_map(&self, vars: &'a VarjMap) -> Option<&'a str> {
        vars.get(self.variable_key)
    }
}

fn generate_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut in_block = false;
    let mut idx_start = 0;
    let mut line = 1;
    let mut line_start = 1;
    let mut col = 0;
    let mut col_start = 0;

    let mut chars = input.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        col += 1;

        if ch == '\n' {
            line += 1;
            col = 0;
        }

        if in_block && ch == '}' {
            match chars.peek() {
                Some((next_idx, next_ch)) if *next_ch == '}' => {
                    blocks.push(Block {
                        start: idx_start,
                        len: next_idx - idx_start + 1,
                        line: line_start,
                        col: col_start,
                        variable_key: input[idx_start + 2..next_idx - 1].trim(),
                    });

                    // end of block
                    in_block = false;
                    col += 1;
                    chars.next();
                }
                Some(_) => continue,
                None => break,
            };
        } else if ch == '{' {
            match chars.peek() {
                Some((_, next_ch)) if *next_ch == '{' => {
                    // start of block
                    idx_start = idx;
                    line_start = line;
                    col_start = col;
                    in_block = true;
                    col += 1;
                    chars.next();
                }
                Some(_) => continue,
                None => break,
            };
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_var() {
        test_parse_vars(
            "testKey: testValue;",
            "testKey: {{ testKey }};",
            &[("testKey", "testValue")],
        );
    }

    #[test]
    fn parse_multiple_vars() {
        test_parse_vars(
            "testKey: testValue; testKey2: testValue2;",
            "testKey: {{testKey}}; testKey2: {{ testKey2 }};",
            &[("testKey", "testValue"), ("testKey2", "testValue2")],
        );
    }

    #[test]
    fn parse_without_vars() {
        test_parse_vars(
            "testKey: testValue; testKey2: testValue2;",
            "testKey: testValue; testKey2: testValue2;",
            &[],
        );
    }

    #[test]
    fn parse_incorrect_vars() {
        let input = "testKey: {{testKey}}; testValue2: {{ wrongKey }};";
        let mut map = VarjMap::new();
        map.insert("testKey", "testValue");
        map.insert("testKey2", "testValue2");

        let expected = Error {
            line: 1,
            col: 35,
            key: "wrongKey".to_owned(),
        };

        let actual = map.parse(input).expect_err("parsing should error");
        assert_eq!(expected.line, actual.line);
        assert_eq!(expected.col, actual.col);
        assert_eq!(expected.key, actual.key);

        let expected_error_msg = format!(
            "{}:{} unknown variable '{}'",
            expected.line, expected.col, expected.key
        );
        let actual_error_msg = format!("{}", actual);
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    fn generate_single_block_with_whitespace() {
        test_generated_blocks(
            "testKey: {{ testKey }};",
            vec![Block {
                start: 9,
                len: 13,
                line: 1,
                col: 10,
                variable_key: "testKey",
            }],
        );
    }

    #[test]
    fn generate_single_block_without_whitespace() {
        test_generated_blocks(
            "testKey: {{testKey}};",
            vec![Block {
                start: 9,
                len: 11,
                line: 1,
                col: 10,
                variable_key: "testKey",
            }],
        );
    }

    #[test]
    fn generate_single_block_at_start() {
        test_generated_blocks(
            "{{testKey}}: testKey",
            vec![Block {
                start: 0,
                len: 11,
                line: 1,
                col: 1,
                variable_key: "testKey",
            }],
        );
    }

    #[test]
    fn generate_single_block_at_len() {
        test_generated_blocks(
            "testKey: {{testKey}}",
            vec![Block {
                start: 9,
                len: 11,
                line: 1,
                col: 10,
                variable_key: "testKey",
            }],
        );
    }

    #[test]
    fn generate_single_block_with_added_braces() {
        test_generated_blocks(
            "test{Key: {{ test}Key }};",
            vec![Block {
                start: 10,
                len: 14,
                line: 1,
                col: 11,
                variable_key: "test}Key",
            }],
        );
    }

    #[test]
    fn generate_multiple_blocks() {
        test_generated_blocks(
            "testKey: {{testKey}}; testKey2: {{ testKey2 }};",
            vec![
                Block {
                    start: 9,
                    len: 11,
                    line: 1,
                    col: 10,
                    variable_key: "testKey",
                },
                Block {
                    start: 32,
                    len: 14,
                    line: 1,
                    col: 33,
                    variable_key: "testKey2",
                },
            ],
        );
    }

    #[test]
    fn generate_multiple_blocks_on_multiple_lines() {
        test_generated_blocks(
            "testKey: {{testKey}};\ntestKey2: {{ testKey2 }};",
            vec![
                Block {
                    start: 9,
                    len: 11,
                    line: 1,
                    col: 10,
                    variable_key: "testKey",
                },
                Block {
                    start: 32,
                    len: 14,
                    line: 2,
                    col: 11,
                    variable_key: "testKey2",
                },
            ],
        );
    }

    #[test]
    fn from_hash_map() {
        let (expected, hash_map) = matching_varj_and_hash_maps();
        let actual = VarjMap::from(hash_map);
        assert_eq!(expected, actual);
    }

    #[test]
    fn into_hash_map() {
        let (varj_map, expected) = matching_varj_and_hash_maps();
        let actual = HashMap::from(varj_map);
        assert_eq!(expected, actual);
    }

    fn test_parse_vars(expected: &str, input: &str, vars: &[(&str, &str)]) {
        let mut map = VarjMap::new();
        for (k, v) in vars {
            map.insert(*k, *v);
        }
        let actual = map.parse(input).expect("parsing should succeed");
        assert_eq!(expected, actual);
    }

    fn test_generated_blocks(input: &str, expected: Vec<Block>) {
        let actual = generate_blocks(input);
        for (idx, _block) in actual.iter().enumerate() {
            assert_eq!(expected[idx], actual[idx]);
        }
    }

    fn matching_varj_and_hash_maps() -> (VarjMap, HashMap<String, String>) {
        let key1 = "testKey1";
        let value1 = "testValue1";

        let key2 = "testKey2";
        let value2 = "testValue2";

        let mut hash_map = HashMap::with_capacity(2);
        hash_map.insert(key1.to_string(), value1.to_string());
        hash_map.insert(key2.to_string(), value2.to_string());

        let mut varj_map = VarjMap::with_capacity(2);
        varj_map.insert(key1, value1);
        varj_map.insert(key2, value2);

        (varj_map, hash_map)
    }
}
