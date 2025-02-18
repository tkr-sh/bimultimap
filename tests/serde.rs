#[cfg(feature = "serde")]
mod deser {
    use bimultimap::BiMultiMap;

    #[test]
    fn basic() {
        let simple = r#"{"a":"b"}"#;

        let map: BiMultiMap<&'static str, &'static str> = serde_json::from_str(simple).unwrap();

        assert_eq!(map, BiMultiMap::from_iter([("a", "b")]))
    }

    #[test]
    fn multiple_left_right() {
        let simple = r#"{"a":"b", "a":"a", "b":"b", "b":"a"}"#;

        let map: BiMultiMap<&'static str, &'static str> = serde_json::from_str(simple).unwrap();

        assert_eq!(
            map,
            BiMultiMap::from_iter([
                ("a", "b"),
                ("a", "a"),
                ("b", "b"),
                ("b", "a"),
            ])
        )
    }

    #[test]
    fn str_and_int() {
        let simple = r#"{"a":0, "a":1}"#;

        let map: BiMultiMap<&'static str, u8> = serde_json::from_str(simple).unwrap();

        assert_eq!(map, BiMultiMap::from_iter([("a", 0), ("a", 1),]))
    }
}

#[cfg(feature = "serde")]
mod ser {
    use bimultimap::BiMultiMap;

    #[test]
    fn basic() {
        let str = r#"{"a":"b"}"#;
        let map = BiMultiMap::from_iter([("a", "b")]);
        let result = serde_json::to_string(&map);
        assert_eq!(str, result.unwrap())
    }

    #[test]
    fn str_and_int() {
        let str = r#"{"a":0}"#;
        let map = BiMultiMap::from_iter([("a", 0)]);
        let result = serde_json::to_string(&map);
        assert_eq!(str, result.unwrap())
    }
}
