use std::error::Error;
use varj::VarjMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut variables = VarjMap::new();

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
    Ok(())
}
