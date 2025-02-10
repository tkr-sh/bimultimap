mod into_iter {
    use bimultimap::BiMultiMap;
    use bimultimap::Rc;

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
    use std::{hash::Hash, sync::Mutex};

    use bimultimap::{BiMultiMap, Rc};
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
        let mut map = BiMultiMap::<String, String>::new();

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
