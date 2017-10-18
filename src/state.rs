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
#[derive(Debug, PartialEq)]
pub enum OpErr {
    EmptyKey,
    NotEnoughArgs(usize),
}

#[derive(Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
enum Match<T> {
    FullMatch(T),
    PartialMatch,
    NoMatch,
}

/// Matches a sequence `Vec<K>` using Vi's mapping rules:
/// * First check for partial matches such that `query` is shorter than the key.
///   In this case, return `Match::PartialMatch`.
/// * If no partial match is found, check for a full match. In this
///   case, return the value it maps to in a `Match::FullMatch` variant.
/// * If no partial or full match is found, return `Match::NoMatch`.
fn find_match<'a, K, T>(
    map: &'a OrderedVecMap<Vec<K>, T>,
    query: &Vec<K>,
) -> Match<&'a T>
where
    K: Ord,
{
    if query.is_empty() {
        return Match::NoMatch;
    }

    let partial_matcher = |probe: &(Vec<K>, T)| match probe.0.cmp(query) {
        Less => Less,
        Equal => Less, // When searching for partial matches, ignore equal.
        Greater => {
            return if probe.0.len() > query.len() &&
                probe.0.starts_with(query)
            {
                Equal
            } else {
                Greater
            };
        }
    };

    map.find_by(partial_matcher).map_or(
        map.find(query).map_or(
            Match::NoMatch, // No partial or full match found.
            |full| Match::FullMatch(full), // No partial match, found full.
        ),
        |_| Match::PartialMatch, // Found a partial match.
    )
}

/// If `query` is a full match to a key found in `map`, the
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
        // TODO no longer need to incrementally add elements to the query.
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
    #[test]
    fn partial_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(Match::PartialMatch, find_match(&map, &query))
    }

    #[test]
    fn full_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(Match::FullMatch(&6u8), find_match(&map, &query))
    }

    #[test]
    fn best_full_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8], 4u8));
        map.insert((vec![1u8, 2u8], 5u8));
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_eq!(Match::FullMatch(&6u8), find_match(&map, &query))
    }

    #[test]
    fn no_match() {
        let mut map = OrderedVecMap::<Vec<u8>, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![2u8, 3u8];
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
mod mode_map {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestOp {
        ThingOne,
        ThingTwo,
    }

    #[test]
    fn process_one_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_two_ops() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(2u8);

        assert_eq!(Some(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert!(typeahead.is_empty());
    }

    #[test]
    fn process_put_back_leftovers() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_overspecified_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8, 1u8], vec![2u8])
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_remap_then_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![2u8], TestOp::ThingTwo)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);

        assert_eq!(Some(TestOp::ThingTwo), mode_map.process(&mut typeahead));
        assert_eq!(Some(1u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }
}
