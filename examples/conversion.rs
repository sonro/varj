use std::{collections::HashMap, error::Error};
use varj::{CowHashMap, VarjMap};

fn main() -> Result<(), Box<dyn Error>> {
    // create a vector of string pairs
    let pairs = vec![("key1", "value1"), ("key2", "value2")];

    // create a HashMap by iterating over a vector or string pairs
    let hash_map: HashMap<&str, &str> = pairs.into_iter().collect();

    // convert the HashMap into a VarjMap
    let map = VarjMap::from(hash_map);

    // use it to render a template
    let expected = "value1";
    let actual = map.render("{{ key1 }}")?;
    assert_eq!(expected, actual);

    // convert it back into a HashMap
    let hash_map: CowHashMap = map.into();
    let expected = "value2";
    let actual = hash_map.get("key2").unwrap();
    assert_eq!(expected, actual);

    Ok(())
}
