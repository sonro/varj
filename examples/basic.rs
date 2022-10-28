use std::error::Error;
use varj::VarjMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = VarjMap::new();
    map.insert("key", "value");

    let expected = "value";
    let actual = map.render("{{ key }}")?;

    assert_eq!(expected, actual);
    Ok(())
}
