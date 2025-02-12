mod into_iter {
    use bimultimap::{BiMultiMap, Rc};

    #[test]
    pub fn basic() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("b", "b");

        let mut imap = map.into_iter();

        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_none());
    }

    #[test]
    pub fn no_value() {
        let map = BiMultiMap::<String, String>::new();

        let mut imap = map.into_iter();

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn lot_of_values() {
        let mut map = BiMultiMap::new();

        const ITERS: usize = 10_000;

        for i in 0..ITERS {
            map.insert(i, 0);
        }

        let mut imap = map.into_iter();

        for _ in 0..ITERS {
            assert!(imap.next().is_some());
        }

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn one_value_test() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");

        let mut imap = map.into_iter();

        assert_eq!(imap.next(), Some((Rc::new("a"), Rc::new("b"),)));
    }
}

mod iter_ref {
    use bimultimap::BiMultiMap;
    #[test]
    pub fn basic() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("b", "b");

        let mut imap = map.iter_ref();

        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_none());
    }

    #[test]
    pub fn no_value() {
        let map = BiMultiMap::<String, String>::new();

        let mut imap = map.iter_ref();

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn lot_of_values() {
        let mut map = BiMultiMap::new();

        const ITERS: usize = 10_000;

        for i in 0..ITERS {
            map.insert(i, 0);
        }

        let mut imap = map.iter_ref();

        for _ in 0..ITERS {
            assert!(imap.next().is_some());
        }

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn one_value_test() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");

        let mut imap = map.iter_ref();

        assert_eq!(imap.next(), Some((&"a", &"b")));
    }
}

mod iter {
    use {
        bimultimap::{BiMultiMap, Rc},
        std::{hash::Hash, sync::Mutex},
    };

    #[test]
    pub fn basic() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");
        map.insert("a", "c");
        map.insert("b", "b");

        let mut imap = map.iter();

        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_some());
        assert!(imap.next().is_none());
    }

    #[test]
    pub fn no_value() {
        let map = BiMultiMap::<String, String>::new();

        let mut imap = map.iter();

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn lot_of_values() {
        let mut map = BiMultiMap::new();

        const ITERS: usize = 10_000;

        for i in 0..ITERS {
            map.insert(i, 0);
        }

        let mut imap = map.iter();

        for _ in 0..ITERS {
            assert!(imap.next().is_some());
        }

        assert!(imap.next().is_none());
    }

    #[test]
    pub fn one_value_test() {
        let mut map = BiMultiMap::new();

        map.insert("a", "b");

        let mut imap = map.iter();

        assert_eq!(imap.next(), Some((&Rc::new("a"), &Rc::new("b"))));
    }

    #[test]
    pub fn modifying_values() {
        #[derive(Debug)]
        struct HashMutex(Mutex<i32>);

        impl std::cmp::PartialEq for HashMutex {
            fn eq(&self, other: &Self) -> bool {
                *self.0.lock().unwrap() == *other.0.lock().unwrap()
            }
        }

        impl Eq for HashMutex {
            fn assert_receiver_is_total_eq(&self) {}
        }

        impl Hash for HashMutex {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                state.write_i32(*self.0.lock().unwrap());
            }
        }

        let mut map = BiMultiMap::new();

        map.insert(HashMutex(Mutex::new(3)), 2);
        map.insert(HashMutex(Mutex::new(4)), 2);
        map.insert(HashMutex(Mutex::new(5)), 2);

        let imap = map.iter();

        for (l, _) in imap {
            if *l == Rc::new(HashMutex(Mutex::new(3))) {
                *l.0.lock().unwrap() = 10;
            }
        }

        println!("{:#?}", map);
        assert!(map.iter().any(|(l, _)| *l.0.lock().unwrap() == 10))
    }
}

mod from_iter {
    use {
        bimultimap::{BiMultiMap, Rc},
        hashbrown::HashSet,
    };

    #[test]
    fn one_iter() {
        let map = BiMultiMap::from_iter([(0, 1)]);

        assert_eq!(map.get_right(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&0), Some(&HashSet::from([Rc::new(1)])));
    }

    #[test]
    fn multiple_iter() {
        let map = BiMultiMap::from_iter([(0, 1), (1, 2), (2, 1), (1, 1)]);

        assert_eq!(map.get_left(&0), Some(&HashSet::from([Rc::new(1)])));
        assert_eq!(
            map.get_left(&1),
            Some(&HashSet::from([Rc::new(1), Rc::new(2)]))
        );
        assert_eq!(map.get_left(&2), Some(&HashSet::from([Rc::new(1)])));

        assert_eq!(
            map.get_right(&1),
            Some(&HashSet::from([Rc::new(0), Rc::new(1), Rc::new(2)]))
        );
        assert_eq!(map.get_right(&2), Some(&HashSet::from([Rc::new(1)])));
    }

    #[test]
    fn reverse_left_right() {
        let map = BiMultiMap::from_iter([(0, 1)].into_iter().map(|(a, b)| (b, a)));
        assert_eq!(map.get_left(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&0), Some(&HashSet::from([Rc::new(1)])));
    }

    #[test]
    fn repeating() {
        let map = BiMultiMap::from_iter([(0, 0), (0, 0)]);
        assert_eq!(map.get_left(&0), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&0), Some(&HashSet::from([Rc::new(0)])));
    }

    #[test]
    fn lot_of_values() {
        const ITERS: u32 = 10_000;
        let map = BiMultiMap::from_iter((0..ITERS).map(|v| (v, v)));

        let mut imap = map.iter();

        for _ in 0..ITERS {
            assert!(imap.next().is_some());
        }

        assert!(imap.next().is_none());
    }
}
