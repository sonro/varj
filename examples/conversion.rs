use std::{collections::HashMap, error::Error};
use varj::VarjMap;

fn main() -> Result<(), Box<dyn Error>> {
    // create a vector of string pairs
    let pairs = vec![
        ("key1".to_owned(), "value1".to_owned()),
        ("key2".to_owned(), "value2".to_owned()),
    ];

    // create a HashMap by iterating over a vector or string pairs
    let hash_map: HashMap<String, String> = pairs.into_iter().collect();

    // convert the HashMap into a VarjMap
    let map = VarjMap::from(hash_map);

    // use it to parse a template
    let expected = "value1";
    let actual = map.parse("{{ key1 }}")?;
    assert_eq!(expected, actual);

    // convert it back into a HashMap
    let hash_map: HashMap<String, String> = map.into();
    let expected = "value2";
    let actual = hash_map.get("key2").unwrap();
    assert_eq!(expected, actual);

    Ok(())
}
