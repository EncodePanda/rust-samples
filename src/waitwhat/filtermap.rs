use super::just::Maybe::{Just, Nothing};
use super::wrapper::*;

#[allow(dead_code)]
pub fn do_filter_map(till: u64) -> impl Iterator<Item = u64> {
    (1..till).filter(approved).map(modify)
}

#[allow(dead_code)]
pub fn do_flat_map_option(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| if approved(&n) { Some(modify(n)) } else { None })
}

#[allow(dead_code)]
pub fn do_flat_map_vec(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| {
        if approved(&n) {
            vec![modify(n)]
        } else {
            vec![]
        }
    })
}

#[allow(dead_code)]
pub fn do_flat_map_array(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| if approved(&n) { [modify(n)] } else { [n] })
}

#[allow(dead_code)]
pub fn do_flat_map_wrapper(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| -> _ {
        if approved(&n) {
            Wrapped::new(modify(n))
        } else {
            Wrapped::new(n)
        }
    })
}

#[allow(dead_code)]
pub fn do_flat_map_maybe(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| {
        if approved(&n) {
            Just(modify(n))
        } else {
            Nothing
        }
    })
}

pub fn approved(n: &u64) -> bool {
    n % 2 == 0
}

pub fn modify(n: u64) -> u64 {
    n * 10
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_filter_map_and_flat_map() {
        let expected = vec![20, 40, 60, 80];
        assert_eq!(do_filter_map(10).collect::<Vec<_>>(), expected);
        assert_eq!(do_flat_map_option(10).collect::<Vec<_>>(), expected);
        assert_eq!(do_flat_map_vec(10).collect::<Vec<_>>(), expected);
        assert_eq!(do_flat_map_maybe(10).collect::<Vec<_>>(), expected);
        let expected = vec![1, 20, 3, 40, 5, 60, 7, 80, 9];
        assert_eq!(do_flat_map_array(10).collect::<Vec<_>>(), expected);
        assert_eq!(do_flat_map_wrapper(10).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_hint_size_filter_map_and_flat_map() {
        assert_eq!(do_filter_map(10).size_hint(), (0, Some(9)));
        assert_eq!(do_flat_map_option(10).size_hint(), (0, Some(9)));
        assert_eq!(do_flat_map_vec(10).size_hint(), (0, None));
        assert_eq!(do_flat_map_maybe(10).size_hint(), (0, None));
        assert_eq!(do_flat_map_wrapper(10).size_hint(), (0, Some(9)));
        assert_eq!(do_flat_map_array(10).size_hint(), (9, Some(9)));
    }
}
