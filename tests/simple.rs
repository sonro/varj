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

    test_render(json, map, expected);
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

    test_render(json, map, expected);
}

#[test]
fn owned_data() {
    fn create_map<'a>() -> VarjMap<'a> {
        let mut map = VarjMap::new();
        let key = String::from("key");
        let value = String::from("value");
        map.insert(key, value);
        map
    }

    let map = create_map();
    test_render("{{ key }}", map, "value");
}

#[test]
fn into_string_into_cow_regression() {
    fn insert_str<K, V>(map: &mut VarjMap, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        map.insert(key.into(), value.into());
    }
    let mut map = VarjMap::new();
    let key = "key";
    let value = "value";

    insert_str(&mut map, key, value);
    test_render("{{ key }}", map, "value");
}

fn test_render(template: &str, map: VarjMap, expected: &str) {
    let actual = map.render(template).expect("rendering should succeed");
    assert_eq!(expected, actual);
}
