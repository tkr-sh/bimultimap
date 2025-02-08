mod remove_right_and_left {
    use bimultimap::{BiMultiMap, Rc};
    use hashbrown::HashSet;

    #[test]
    pub fn basic_remove() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.remove("a", "b");

        assert_eq!(map.get_left(&"a"), None)
    }

    #[test]
    pub fn remove_with_some_result() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.remove("a", "b");

        assert_eq!(map.get_left(&"a"), Some(&HashSet::from([Rc::new("c")])));
    }

    #[test]
    pub fn remove_with_more_results() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("a", "d");
        map.insert("a", "e");
        map.remove("a", "b");
        map.remove("a", "d");

        assert_eq!(
            map.get_left(&"a"),
            Some(&HashSet::from([Rc::new("c"), Rc::new("e")]))
        );
    }
}
