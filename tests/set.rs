mod set_left {
    use bimultimap::{BiMultiMap, HashSet, Rc};

    #[test]
    pub fn basic() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.set_left(0.into(), [1, 2, 3].into());

        assert_eq!(
            map.get_left(&0),
            Some(&HashSet::from([Rc::new(1), Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_right(&0), None);
        assert_eq!(map.get_right(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&2), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&3), Some(&HashSet::from([Rc::new(0)])));
    }

    #[test]
    pub fn setting_already_existing_values() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.insert(0, 4);
        map.set_left(0.into(), [1, 2, 3].into());

        assert_eq!(
            map.get_left(&0),
            Some(&HashSet::from([Rc::new(1), Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_right(&0), None);
        assert_eq!(map.get_right(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&2), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&3), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_right(&4), None);
    }

    #[test]
    pub fn multiple_both_side_keys() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.insert(0, 2);
        map.insert(0, 3);
        map.insert(1, 2);
        map.insert(1, 3);
        map.set_left(0.into(), [4].into());

        assert_eq!(map.get_left(&0), Some(&HashSet::from([Rc::new(4)])));
        assert_eq!(
            map.get_left(&1),
            Some(&HashSet::from([Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_right(&2), Some(&HashSet::from([Rc::new(1)])));
        assert_eq!(map.get_right(&3), Some(&HashSet::from([Rc::new(1)])));
        assert_eq!(map.get_right(&4), Some(&HashSet::from([Rc::new(0)])));
    }
}

mod set_right {
    use bimultimap::{BiMultiMap, HashSet, Rc};

    #[test]
    pub fn basic() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.set_right(0.into(), [1, 2, 3].into());

        assert_eq!(
            map.get_right(&0),
            Some(&HashSet::from([Rc::new(1), Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_left(&0), None);
        assert_eq!(map.get_left(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&2), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&3), Some(&HashSet::from([Rc::new(0)])));
    }

    #[test]
    pub fn setting_already_existing_values() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.insert(4, 0);
        map.set_right(0.into(), [1, 2, 3].into());

        assert_eq!(
            map.get_right(&0),
            Some(&HashSet::from([Rc::new(1), Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_left(&0), None);
        assert_eq!(map.get_left(&1), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&2), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&3), Some(&HashSet::from([Rc::new(0)])));
        assert_eq!(map.get_left(&4), None);
    }

    #[test]
    pub fn multiple_both_side_keys() {
        let mut map = BiMultiMap::<i32, i32>::new();

        map.insert(2, 0);
        map.insert(3, 0);
        map.insert(2, 1);
        map.insert(3, 1);
        map.set_right(0.into(), [4].into());

        assert_eq!(map.get_right(&0), Some(&HashSet::from([Rc::new(4)])));
        assert_eq!(
            map.get_right(&1),
            Some(&HashSet::from([Rc::new(2), Rc::new(3),]))
        );

        assert_eq!(map.get_left(&2), Some(&HashSet::from([Rc::new(1)])));
        assert_eq!(map.get_left(&3), Some(&HashSet::from([Rc::new(1)])));
        assert_eq!(map.get_left(&4), Some(&HashSet::from([Rc::new(0)])));
    }
}
