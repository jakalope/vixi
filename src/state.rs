use common::{prepend, move_to_front};
use op::{NormalOp, InsertOp};
use ordered_vec_map::{InsertionResult, OrderedVecMap};
use std::cmp::Ord;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::VecDeque;

/// By convention:
/// * K is a map's key type, implementing `Copy` and `Ord`.
/// * T is an arbitrary type, typically stored as a value in a map.
/// * Op is an arbitrary operation type (typically a mode-specific enum).

pub enum OpErr {
    EmptyKey,
    NotEnoughArgs(usize),
}

pub enum RemapErr {
    EmptyKey,
}

pub struct ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
{
    // TODO Handle noremap (key,value) by surrounding value with non-input-able
    // keys, so if it gets put in the typeahead, it cannot possibly be remapped.
    // This would also mean such values would be ignored by the op-map.
    key_map: OrderedVecMap<Vec<K>, Vec<K>>,
    op_map: OrderedVecMap<Vec<K>, Op>,
}

pub struct State<K>
where
    K: Ord,
    K: Copy,
{
    pub typeahead: VecDeque<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
}

enum Match<T> {
    FullMatch(T),
    PartialMatch,
    NoMatch,
}

/// Matches a sequence `Vec<K>` using Vi's mapping rules:
/// * First check for partial matches such that `query` is shorter than the key.
///   In this case, return `Match::PartialMatch`.
/// * If no partial match is found, check for a full (exact) match. In this
///   case, return the value it maps to in a `Match::FullMatch` variant.
/// * If no partial or full match is found, return `Match::NoMatch`.
fn find_match<'a, K, T>(
    map: &'a OrderedVecMap<Vec<K>, T>,
    query: &Vec<K>,
) -> Match<&'a T>
where
    K: Ord,
{
    let partial_matcher = |probe: &(Vec<K>, T)| if probe.0.len() >
        query.len() &&
        probe.0.starts_with(query)
    {
        Equal // Found a partial match.
    } else {
        match probe.0.cmp(query) {
            Less => Less,
            Equal => Less, // When searching for partial matches, ignore equal.
            Greater => Greater,
        }
    };

    if query.is_empty() {
        return Match::NoMatch;
    }

    map.find_by(partial_matcher).map_or(
        map.find(query).map_or(
            Match::NoMatch, // No partial or full match found.
            |full| Match::FullMatch(full), // No partial match, found full.
        ),
        |_| Match::PartialMatch, // Found a partial match.
    )
}

/// If `query` is a full (exact) match to a key found in `map`, the
/// corresponding map value is prepended to the typeahead buffer.
/// Returns `true` if a full or partial match is found. Otherwise, `false`.
fn remap<K>(
    map: &OrderedVecMap<Vec<K>, Vec<K>>,
    query: &mut Vec<K>,
    typeahead: &mut VecDeque<K>,
) -> bool
where
    K: Ord,
    K: Copy,
{
    match find_match(map, query) {
        Match::FullMatch(mapped_keys) => {
            // Put mapped keys in front of the typeahead buffer.
            for key in mapped_keys.iter().rev() {
                typeahead.push_front(*key);
            }
            // Clear query (ergo, we'll skip matching noremap and op until next
            // iteration).
            query.clear();
            true
        }
        Match::PartialMatch => {
            // Keep searching.
            true
        }
        Match::NoMatch => {
            // We're done searching this map.
            false
        }
    }
}

impl<K, Op> ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
    pub fn new() -> Self {
        ModeMap {
            key_map: OrderedVecMap::<Vec<K>, Vec<K>>::new(),
            op_map: OrderedVecMap::<Vec<K>, Op>::new(),
        }
    }

    // Loop until a partly matching mapping is found or all (local) mappings
    // have been checked.  The longest full match is remembered in "mp_match".
    // A full match is only accepted if there is no partly match, so "aa" and
    // "aaa" can both be mapped.
    // https://github.com/vim/vim/blob/master/src/getchar.c#L2140-L2146
    pub fn process(&self, mut typeahead: &mut VecDeque<K>) -> Option<Op> {
        // Grab incrementally more keys from the front of the queue, looking for
        // matches.
        let mut query = Vec::<K>::with_capacity(typeahead.len());
        let mut mapping = true;
        let mut opping = true;
        while !typeahead.is_empty() && (mapping || opping) {
            query.push(typeahead.pop_front().unwrap());
            mapping = mapping &&
                remap(&self.key_map, &mut query, &mut typeahead);
            match find_match(&self.op_map, &mut query) {
                Match::FullMatch(op) => {
                    return Some(*op);
                }
                Match::NoMatch => {
                    opping = false;
                }
                Match::PartialMatch => {}
            }
        }
        // Put whatever is left back in the typeahead buffer.
        move_to_front(&mut query, typeahead);
        None
    }

    pub fn insert_op(
        &mut self,
        key: Vec<K>,
        value: Op,
    ) -> Result<InsertionResult, OpErr> {
        // Disallow empty keys, as they would full-match against an empty
        // typeahead buffer.
        if key.is_empty() {
            return Err(OpErr::EmptyKey);
        }
        return Ok(self.op_map.insert((key, value)));
    }

    pub fn insert_remap(
        &mut self,
        key: Vec<K>,
        value: Vec<K>,
    ) -> Result<InsertionResult, RemapErr> {
        // Disallow empty keys, as they would full-match against an empty
        // typeahead buffer.
        if key.is_empty() {
            return Err(RemapErr::EmptyKey);
        }
        return Ok(self.key_map.insert((key, value)));
    }
}

impl<K> State<K>
where
    K: Ord,
    K: Copy,
{
    pub fn new() -> Self {
        State {
            typeahead: VecDeque::<K>::new(),
            normal_mode_map: ModeMap::<K, NormalOp>::new(),
            insert_mode_map: ModeMap::<K, InsertOp>::new(),
        }
    }
}

#[cfg(test)]
mod remap {
    use super::*;

    #[test]
    fn no_match() {
        let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
        map.insert((vec![2u8, 3u8], vec![6u8]));
        let mut query = vec![1u8, 2u8, 3u8];
        let mut typeahead = VecDeque::<u8>::new();
        assert_eq!(false, remap(&map, &mut query, &mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn partial_match() {
        let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], vec![6u8]));
        let mut query = vec![1u8, 2u8, 3u8];
        let mut typeahead = VecDeque::<u8>::new();
        assert_eq!(true, remap(&map, &mut query, &mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn full_match() {
        let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
        map.insert((vec![1u8, 2u8, 3u8], vec![6u8]));
        let mut query = vec![1u8, 2u8, 3u8];
        let mut typeahead = VecDeque::<u8>::new();
        assert_eq!(true, remap(&map, &mut query, &mut typeahead));
        assert_eq!(VecDeque::from(vec![6u8]), typeahead);
    }
}

#[cfg(test)]
mod find_match {
    use super::*;
    use std::fmt::Debug;

    fn assert_partial_match<T>(m: Match<T>) {
        match m {
            Match::PartialMatch => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    fn assert_full_match<T>(expected: T, m: Match<T>)
    where
        T: PartialEq,
        T: Debug,
    {
        match m {
            Match::FullMatch(x) => {
                assert_eq!(expected, x);
            }
            _ => {
                assert!(false);
            }
        }
    }

    fn assert_no_match<T>(m: Match<T>) {
        match m {
            Match::NoMatch => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn partial_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_partial_match(find_match(&map, &query))
    }

    #[test]
    fn full_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_full_match(&6u8, find_match(&map, &query))
    }

    #[test]
    fn no_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![2u8, 3u8];
        assert_no_match(find_match(&map, &query))
    }

    #[test]
    fn no_query() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![];
        assert_no_match(find_match(&map, &query))
    }
}
