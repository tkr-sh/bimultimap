mod remove_right_and_left {
    use {
        bimultimap::{BiMultiMap, Rc},
        hashbrown::HashSet,
    };

    #[test]
    pub fn basic_remove() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.remove("a", "b");

        assert_eq!(map.get_left(&"a"), None);
        assert_eq!(map.get_right(&"b"), None);
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

    #[test]
    pub fn remove_until_none() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("b", "b");
        map.insert("b", "c");

        assert_eq!(
            map.get_left(&"a"),
            Some(&HashSet::from([Rc::new("b"), Rc::new("c")]))
        );

        map.remove("a", "b");
        map.remove("a", "c");

        assert_eq!(map.get_left(&"a"), None,);
        assert_eq!(
            map.get_left(&"b"),
            Some(&HashSet::from([Rc::new("b"), Rc::new("c")]))
        );
    }
}

mod remove_left {
    use {
        bimultimap::{BiMultiMap, Rc},
        hashbrown::HashSet,
    };

    #[test]
    pub fn basic_remove() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        assert_eq!(map.get_one_left(&"a"), Some(&"b"));
        assert_eq!(map.get_one_right(&"b"), Some(&"a"));

        let removed = map.remove_left("a");

        assert_eq!(removed, Some(HashSet::from([Rc::new("b")])));

        assert_eq!(map.get_left(&"a"), None);
        assert_eq!(map.get_right(&"b"), None);
    }

    #[test]
    pub fn multiple_rights() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");

        let removed = map.remove_left("a");

        assert_eq!(removed, Some(HashSet::from([Rc::new("b"), Rc::new("c")])));

        assert_eq!(map.get_left(&"a"), None);
    }

    #[test]
    pub fn left_and_rights() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("b", "b");
        map.insert("b", "c");

        let removed = map.remove_left("a");

        assert_eq!(removed, Some(HashSet::from([Rc::new("b"), Rc::new("c")])));

        assert_eq!(map.get_left(&"a"), None);

        assert_eq!(
            map.get_left(&"b"),
            Some(&HashSet::from([Rc::new("b"), Rc::new("c")]))
        );
    }

    #[test]
    pub fn no_removed_values() {
        let mut map = BiMultiMap::new();

        map.insert("a", "_");
        map.insert("b", "_");
        map.insert("c", "_");

        assert_eq!(map.remove_left("d"), None)
    }
}
