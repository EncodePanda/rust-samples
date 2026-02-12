#[allow(dead_code)]
pub fn do_filter_map(till: u64) -> impl Iterator<Item = u64> {
    (1..till).filter(approved).map(modify)
}

#[allow(dead_code)]
pub fn do_flat_map(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| if approved(&n) { Some(modify(n)) } else { None })
}

#[allow(dead_code)]
pub fn do_flat_map_v2(till: u64) -> impl Iterator<Item = u64> {
    (1..till).flat_map(|n| {
        if approved(&n) {
            vec![modify(n)]
        } else {
            vec![]
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
        assert_eq!(do_flat_map(10).collect::<Vec<_>>(), expected);
        assert_eq!(do_flat_map_v2(10).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_hint_size_filter_map_and_flat_map() {
        assert_eq!(do_filter_map(10).size_hint(), (0, Some(9)));
        assert_eq!(do_flat_map(10).size_hint(), (0, Some(9)));
        assert_eq!(do_flat_map_v2(10).size_hint(), (0, None));
    }
}
