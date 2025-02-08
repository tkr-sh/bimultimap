use bimultimap::BiMultiMap;
use hashbrown::HashSet;

#[test]
pub fn basic_insert() {
    let mut map = BiMultiMap::new();

    map.insert("a", "b");

    assert_eq!(map.get_left(&"a"), Some(&HashSet::from([&"b"])))
}
