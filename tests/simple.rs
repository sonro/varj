use varj::VarjMap;

#[test]
fn replace_single_variable() {
    let mut map = VarjMap::new();
    map.insert("name", "TestName");

    let json = r#"{
        "name" = "{{ name }}"
    }"#;

    let expected = r#"{
        "name" = "TestName"
    }"#;

    test_parse(json, map, expected);
}

#[test]
fn replace_multiple_variables() {
    let mut map = VarjMap::new();
    map.insert("name", "TestName");
    map.insert("age", "30");

    let json = r#"{
        "name" = "{{ name }}",
        "age" = {{ age }}
    }"#;

    let expected = r#"{
        "name" = "TestName",
        "age" = 30
    }"#;

    test_parse(json, map, expected);
}

fn test_parse(json_template: &str, map: VarjMap, expected: &str) {
    let actual = map.parse(json_template).expect("parsing should succeed");
    assert_eq!(expected, actual);
}
