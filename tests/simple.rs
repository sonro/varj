use std::collections::HashMap;

#[test]
fn replace_single_variable() {
    let mut variables = HashMap::new();
    variables.insert("name".to_owned(), "TestName".to_owned());

    let json = r#"{
        "name" = "{{ name }}"
    }"#;

    let expected = r#"{
        "name" = "TestName"
    }"#;

    test_parse(json, variables, expected);
}

#[test]
fn replace_multiple_variables() {
    let mut variables = HashMap::new();
    variables.insert("name".to_owned(), "TestName".to_owned());
    variables.insert("age".to_owned(), "30".to_owned());

    let json = r#"{
        "name" = "{{ name }}",
        "age" = {{ age }}
    }"#;

    let expected = r#"{
        "name" = "TestName",
        "age" = 30
    }"#;

    test_parse(json, variables, expected);
}

fn test_parse(json_template: &str, variables: HashMap<String, String>, expected: &str) {
    let actual = varj::parse(json_template, &variables).expect("parsing variables");
    assert_eq!(expected, actual);
}
