use std::cmp::Ord;
use std::cmp::Ordering;
use std::convert::From;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InsertionResult {
    Create,
    Overwrite,
}

// Provides an ordered map with a method to query for partial matches.
// This is useful for disambiguation.
pub struct OrderedVecMap<K, T>
where
    K: Ord,
{
    data: Vec<(K, T)>,
}

impl<K, T> OrderedVecMap<K, T>
where
    K: Ord,
{
    pub fn new() -> Self {
        OrderedVecMap { data: Vec::<(K, T)>::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Returns true if the value is inserted, false if overwritten.
    pub fn insert(&mut self, datum: (K, T)) -> InsertionResult {
        match self.data.binary_search_by(|probe| probe.0.cmp(&datum.0)) {
            Ok(idx) => {
                *self.data.get_mut(idx).unwrap() = datum;
                return InsertionResult::Overwrite;
            }
            Err(idx) => {
                self.data.insert(idx, datum);
                return InsertionResult::Create;
            }
        };
    }

    pub fn find(&self, query: &K) -> Option<&T> {
        match self.data.binary_search_by(|probe| probe.0.cmp(query)) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(_) => None,
        }
    }

    pub fn find_by<'a, F>(&self, f: F) -> Option<&T>
    where
        F: Fn(&(K, T)) -> Ordering,
        K: 'a,
        T: 'a,
    {
        match self.data.binary_search_by(f) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(_) => None,
        }
    }
}

impl<K, T> From<Vec<(K, T)>> for OrderedVecMap<K, T>
where
    K: Ord,
{
    fn from(mut data: Vec<(K, T)>) -> OrderedVecMap<K, T> {
        data.sort_by(|a, b| a.0.cmp(&b.0));
        OrderedVecMap { data: data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_empty_len() {
        let x = OrderedVecMap::<u8, u8>::new();
        assert_eq!(0, x.len());
        assert_eq!(true, x.is_empty());
    }

    #[test]
    fn test_insert_one_len() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        match x.insert((4u8, 2u8)) {
            InsertionResult::Overwrite => {
                assert!(false);
            }
            _ => {}
        };
        assert_eq!(1, x.len());
        assert_eq!(false, x.is_empty());
    }

    #[test]
    fn test_insert_two_len() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        match x.insert((4u8, 2u8)) {
            InsertionResult::Overwrite => {
                assert!(false);
            }
            _ => {}
        };
        match x.insert((3u8, 3u8)) {
            InsertionResult::Overwrite => {
                assert!(false);
            }
            _ => {}
        };
        assert_eq!(2, x.len());
    }

    #[test]
    fn test_insert_same_key_len() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        match x.insert((4u8, 2u8)) {
            InsertionResult::Overwrite => {
                assert!(false);
            }
            _ => {}
        };
        match x.insert((3u8, 3u8)) {
            InsertionResult::Overwrite => {
                assert!(false);
            }
            _ => {}
        };
        match x.insert((3u8, 3u8)) {
            InsertionResult::Create => {
                assert!(false);
            }
            _ => {}
        };
        assert_eq!(2, x.len());
    }

    #[test]
    fn test_find_one() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        x.insert((4u8, 2u8));
        assert_eq!(Some(&2u8), x.find(&4u8));
    }

    #[test]
    fn test_find_one_by() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        x.insert((4u8, 2u8));
        assert_eq!(Some(&2u8), x.find_by(|probe| probe.0.cmp(&4u8)));
    }

    #[test]
    fn test_find() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        x.insert((4u8, 2u8));
        x.insert((3u8, 3u8));
        assert_eq!(Some(&2u8), x.find(&4u8));
    }

    #[test]
    fn test_find_by() {
        let mut x = OrderedVecMap::<u8, u8>::new();
        x.insert((3u8, 3u8));
        x.insert((4u8, 2u8));
        assert_eq!(Some(&2u8), x.find_by(|probe| probe.0.cmp(&4u8)));
    }

    #[test]
    fn test_from_vec() {
        let v = vec![(1u8, 3u8), (2u8, 2u8), (3u8, 1u8)];
        let x = OrderedVecMap::<u8, u8>::from(v);
        assert_eq!(3, x.len());
        assert_eq!(Some(&3u8), x.find(&1u8));
    }
}
