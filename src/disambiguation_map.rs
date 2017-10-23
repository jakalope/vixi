use ordered_vec_map::{InsertionResult, OrderedVecMap, RemovalResult};
use std::cmp::{min, max, Ord, Ordering};
use typeahead::{RemapType, Typeahead};

// TODO Handle noremap (key,value) by surrounding value with non-input-able
// keys, so if it gets put in the typeahead, it cannot possibly be remapped.
// This would also mean such values would be ignored by the op-map.
pub struct DisambiguationMap<K, T>
where
    K: Ord,
    K: Copy,
    T: Clone,
{
    // Use an ordered map in order to trade insertion speed for lookup speed.
    vec_map: OrderedVecMap<Vec<K>, T>,
    max_key_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Match<T> {
    FullMatch(T),
    PartialMatch,
    NoMatch,
}

fn match_length<K>(query: &Vec<K>, probe: &Vec<K>) -> usize
where
    K: Ord,
{
    let mut match_len: usize = 0;
    while {
        let p = probe.get(match_len);
        let q = query.get(match_len);
        match_len += 1;
        p.and(q).is_some() && p.cmp(&q) == Ordering::Equal // Loop criteria
    }
    {}
    match_len - 1
}

/// Matches a sequence `Vec<K>` using Vi's mapping rules.
/// ```
/// //  Summary:
/// //    MatchLen < QueryLen <  KeyLen   =>  Not a match
/// //    MatchLen <  KeyLen  < QueryLen  =>  Not a match
/// //    MatchLen <  KeyLen  = QueryLen  =>  Not a match
/// //    MatchLen = QueryLen <  KeyLen   =>  Partial |
/// //    MatchLen =  KeyLen  < QueryLen  =>  Full    |- MatchLen >=
/// //    MatchLen =  KeyLen  = QueryLen  =>  Full    |  min(QueryLen, KeyLen)
/// ```
fn find_match<'a, K, T>(
    map: &'a OrderedVecMap<Vec<K>, T>,
    query: &Vec<K>,
) -> Match<&'a (Vec<K>, T)>
where
    K: Ord,
    K: Copy,
{
    let query_len = query.len();
    let initial: Vec<K>;
    match query.get(0) {
        Some(val) => {
            // Create a vector containing just the first query element.
            initial = vec![*val];
        }
        None => {
            // Query is empty.
            return Match::NoMatch;
        }
    };

    // Start at the first potential match, where keys start with `initial`.
    let mut index = {
        match map.find_idx(&initial) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
    };

    let mut longest_match_key_len: usize = 0;
    let mut longest_match: Option<&(Vec<K>, T)> = None;
    while let Some(kv) = map.get(index) {
        let match_len = match_length(query, &kv.0);
        if match_len == 0 {
            // Stop early if we are guaranteed not to find any more matches.
            break;
        }
        let key_len = kv.0.len();
        if (match_len >= query_len || match_len >= key_len) &&
            key_len > longest_match_key_len
        {
            longest_match = Some(kv);
            longest_match_key_len = key_len;
        }
        index += 1;
    }
    if longest_match_key_len > query_len {
        Match::PartialMatch
    } else if longest_match.is_some() {
        Match::FullMatch(longest_match.unwrap())
    } else {
        Match::NoMatch
    }
}

impl<K, T> DisambiguationMap<K, T>
where
    K: Ord,
    K: Copy,
    T: Clone,
{
    pub fn new() -> Self {
        DisambiguationMap {
            vec_map: OrderedVecMap::new(),
            max_key_len: 0,
        }
    }

    pub fn insert(&mut self, datum: (Vec<K>, T)) -> InsertionResult {
        let key_len = datum.0.len();
        let result = self.vec_map.insert(datum);
        match result {
            InsertionResult::Create => {
                self.max_key_len = max(self.max_key_len, key_len);
            }
            InsertionResult::Overwrite => {}
            InsertionResult::InvalidKey => {}
        }
        result
    }

    pub fn remove(&mut self, key: &Vec<K>) -> RemovalResult {
        let result = self.vec_map.remove(key);
        for kv in self.vec_map.iter() {
            self.max_key_len = max(self.max_key_len, kv.0.len());
        }
        result
    }

    fn fill_query(
        &self,
        typeahead: &Typeahead<K>,
        remap_type: RemapType,
    ) -> Vec<K> {
        // Optimization:
        // Limit query length to no more than longer than longest key.
        let capacity = min(typeahead.len(), self.max_key_len + 1);
        let mut query = Vec::<K>::with_capacity(capacity);
        for k in typeahead.value_iter() {
            query.push(k);
            if query.len() >= query.capacity() {
                break;
            }
        }
        query
    }

    pub fn process(
        &self,
        typeahead: &Typeahead<K>,
        remap_type: RemapType,
    ) -> Match<&(Vec<K>, T)> {
        let query = self.fill_query(typeahead, remap_type);
        find_match(&self.vec_map, &query)
    }
}

#[cfg(test)]
mod find_match {
    use super::*;
    #[test]
    fn partial_match() {
        // MatchLen = QueryLen <  KeyLen   =>  Partial
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(Match::PartialMatch, find_match(&map, &query))
    }

    #[test]
    fn full_match() {
        //    MatchLen =  KeyLen  = QueryLen  =>  Full
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(
            Match::FullMatch(&(vec![1u8, 2u8, 3u8], 6u8)),
            find_match(&map, &query)
        );
    }

    #[test]
    fn overspecified_full_match() {
        //    MatchLen =  KeyLen  < QueryLen  =>  Full
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(
            Match::FullMatch(&(vec![1u8, 2u8], 6u8)),
            find_match(&map, &query)
        );
    }

    #[test]
    fn best_full_match() {
        //    MatchLen =  KeyLen  = QueryLen  =>  Full
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8], 4u8));
        map.insert((vec![1u8, 2u8], 5u8));
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(
            Match::FullMatch(&(vec![1u8, 2u8, 3u8], 6u8)),
            find_match(&map, &query)
        )
    }

    #[test]
    fn underspecified_no_match() {
        //    MatchLen < QueryLen <  KeyLen   =>  Not a match
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![1u8, 3u8];
        assert_eq!(Match::NoMatch, find_match(&map, &query))
    }

    #[test]
    fn overspecified_no_match() {
        //    MatchLen <  KeyLen  < QueryLen  =>  Not a match
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 4u8, 5u8];
        assert_eq!(Match::NoMatch, find_match(&map, &query))
    }

    #[test]
    fn critically_specified_no_match() {
        //    MatchLen <  KeyLen  = QueryLen  =>  Not a match
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 4u8];
        assert_eq!(Match::NoMatch, find_match(&map, &query))
    }

    #[test]
    fn no_query() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![];
        assert_eq!(Match::NoMatch, find_match(&map, &query))
    }
}

#[cfg(test)]
mod match_length {
    use super::*;

    #[test]
    fn exact() {
        let p = Vec::<u8>::from("asdf".as_bytes());
        let q = Vec::<u8>::from("asdf".as_bytes());
        assert_eq!("asdf".as_bytes().len(), match_length(&q, &p));
    }

    #[test]
    fn longer_q() {
        let p = Vec::<u8>::from("asd".as_bytes());
        let q = Vec::<u8>::from("asdf".as_bytes());
        assert_eq!("asd".as_bytes().len(), match_length(&q, &p));
    }

    #[test]
    fn longer_p() {
        let p = Vec::<u8>::from("asdf".as_bytes());
        let q = Vec::<u8>::from("asd".as_bytes());
        assert_eq!("asd".as_bytes().len(), match_length(&q, &p));
    }

    #[test]
    fn both_zero() {
        let p = Vec::<u8>::from("".as_bytes());
        let q = Vec::<u8>::from("".as_bytes());
        assert_eq!(0, match_length(&q, &p));
    }

    #[test]
    fn zero_q() {
        let p = Vec::<u8>::from("asdf".as_bytes());
        let q = Vec::<u8>::from("".as_bytes());
        assert_eq!(0, match_length(&q, &p));
    }
}
