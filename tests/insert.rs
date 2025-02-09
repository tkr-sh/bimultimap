use bimultimap::{BiMultiMap, Rc};
use hashbrown::HashSet;

#[test]
pub fn basic_insert() {
    let mut map = BiMultiMap::new();

    map.insert("a", "b");

    assert_eq!(map.get_left(&"a"), Some(&HashSet::from([Rc::new("b")])))
}

#[test]
pub fn multiple_left_insert() {
    let mut map = BiMultiMap::new();

    map.insert("a", "b");
    map.insert("b", "b");
    map.insert("c", "b");
    map.insert("d", "b");

    assert_eq!(map.get_left(&"a"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_left(&"b"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_left(&"c"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_left(&"d"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(
        map.get_right(&"b"),
        Some(&HashSet::from([
            Rc::new("a"),
            Rc::new("b"),
            Rc::new("c"),
            Rc::new("d"),
        ]))
    );
}

#[test]
pub fn multiple_right_insert() {
    let mut map = BiMultiMap::new();

    map.insert("b", "a");
    map.insert("b", "b");
    map.insert("b", "c");
    map.insert("b", "d");

    assert_eq!(map.get_right(&"a"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_right(&"b"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_right(&"c"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(map.get_right(&"d"), Some(&HashSet::from([Rc::new("b")])));
    assert_eq!(
        map.get_left(&"b"),
        Some(&HashSet::from([
            Rc::new("a"),
            Rc::new("b"),
            Rc::new("c"),
            Rc::new("d"),
        ]))
    );
}

#[test]
pub fn multiple_insert_right_and_left() {
    let mut map = BiMultiMap::new();

    map.insert("a", "a");
    map.insert("a", "b");
    map.insert("b", "a");
    map.insert("b", "b");

    assert_eq!(
        map.get_right(&"b"),
        Some(&HashSet::from([Rc::new("a"), Rc::new("b")]))
    );
    assert_eq!(
        map.get_right(&"a"),
        Some(&HashSet::from([Rc::new("a"), Rc::new("b")]))
    );
    assert_eq!(
        map.get_left(&"b"),
        Some(&HashSet::from([Rc::new("a"), Rc::new("b")]))
    );
    assert_eq!(
        map.get_left(&"a"),
        Some(&HashSet::from([Rc::new("a"), Rc::new("b")]))
    );
}

#[test]
pub fn same_value_insert() {
    let mut map = BiMultiMap::new();

    map.insert("a", "a");
    map.insert("a", "a");
    map.insert("a", "a");
    map.insert("a", "a");

    assert_eq!(map.get_right(&"a"), Some(&HashSet::from([Rc::new("a")])));
}
