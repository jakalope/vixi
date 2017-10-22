use common::{prepend, move_to_front};
use op::{NormalOp, InsertOp};
use ordered_vec_map::{InsertionResult, OrderedVecMap, RemovalResult};
use std::cmp::{min, max};
use std::cmp::Ord;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::VecDeque;
use std::mem::swap;

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
    KeyValueEqual,
}

#[derive(Ord, PartialOrd, Eq, Clone, Debug, PartialEq)]
pub enum MappedObject<K, Op>
where
    K: Ord,
    K: Copy,
{
    Seq(Vec<K>),
    Op(Op),
}

// TODO Handle noremap (key,value) by surrounding value with non-input-able
// keys, so if it gets put in the typeahead, it cannot possibly be remapped.
// This would also mean such values would be ignored by the op-map.
// TODO No need to use an ordered map anymore, since we can't use a binary
// search to speed up the general case for disambiguation.
pub struct ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
    // Use an ordered map in order to trade insertion speed for lookup speed.
    vec_map: OrderedVecMap<Vec<K>, MappedObject<K, Op>>,
    max_key_len: usize,
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

fn match_length<K>(query: &Vec<K>, probe: &Vec<K>) -> usize
where
    K: Ord,
{
    let mut match_len: usize = 0;
    while {
        let p = probe.get(match_len);
        let q = query.get(match_len);
        match_len += 1;
        p.and(q).is_some() && p.cmp(&q) == Equal // Loop criteria
    }
    {}
    match_len - 1
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

/// Matches a sequence `Vec<K>` using Vi's mapping rules.
fn find_match<'a, K, T>(
    map: &'a OrderedVecMap<Vec<K>, T>,
    query: &Vec<K>,
) -> Match<&'a (Vec<K>, T)>
where
    K: Ord,
    K: Copy,
{
    //  Summary:
    //    MatchLen < QueryLen <  KeyLen   =>  Not a match
    //    MatchLen <  KeyLen  < QueryLen  =>  Not a match
    //    MatchLen <  KeyLen  = QueryLen  =>  Not a match
    //    MatchLen = QueryLen <  KeyLen   =>  Partial |
    //    MatchLen =  KeyLen  < QueryLen  =>  Full    |- MatchLen >=
    //    MatchLen =  KeyLen  = QueryLen  =>  Full    |  min(QueryLen, KeyLen)
    let query_len = query.len();
    let mut initial: Vec<K>;
    match query.get(0) {
        Some(val) => {
            initial = vec![*val];
        }
        None => {
            return Match::NoMatch;
        }
    };

    // Start at the first potential match.
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

impl<K, Op> ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
    pub fn new() -> Self {
        ModeMap {
            vec_map: OrderedVecMap::new(),
            max_key_len: 0,
        }
    }

    fn compute_max_key_len(&mut self) {
        for kv in self.vec_map.iter() {
            self.max_key_len = max(self.max_key_len, kv.0.len());
        }
    }

    fn insert(
        &mut self,
        datum: (Vec<K>, MappedObject<K, Op>),
    ) -> InsertionResult {
        let key_len = datum.0.len();
        let result = self.vec_map.insert(datum);
        match result {
            InsertionResult::Create => {
                self.max_key_len = max(self.max_key_len, key_len);
            }
            InsertionResult::Overwrite => {}

        }
        return result;
    }

    pub fn remove(&mut self, key: &Vec<K>) -> RemovalResult {
        let result = self.vec_map.remove(key);
        self.compute_max_key_len();
        return result;
    }

    fn fill_query(&self, typeahead: &mut VecDeque<K>) -> Vec<K> {
        // Optimization:
        // Limit query length to no more than longer than longest key.
        let capacity = min(typeahead.len(), self.max_key_len + 1);
        let mut query = Vec::<K>::with_capacity(capacity);
        for k in typeahead.iter() {
            query.push(*k);
            if query.len() >= query.capacity() {
                break;
            }
        }
        query
    }

    /// Process a typeahead buffer.
    pub fn process(&self, typeahead: &mut VecDeque<K>) -> Option<Op> {
        // Grab keys from the front of the queue, looking for matches.

        let mut i: usize = 0;
        const MAX_ITERATIONS: usize = 1000;
        while typeahead.len() > 0 {
            if i > MAX_ITERATIONS {
                panic!("Infinite loop suspected.");
            }
            i += 1;

            let query = self.fill_query(typeahead);
            match find_match(&self.vec_map, &query) {
                Match::FullMatch(mapped) => {
                    let len = min(mapped.0.len(), typeahead.len());
                    typeahead.drain(..len);
                    match mapped.1 {
                        MappedObject::Seq(ref remapped) => {
                            for k in remapped.iter() {
                                typeahead.push_front(*k);
                            }
                        }
                        MappedObject::Op(op) => {
                            return Some(op);
                        }
                    }
                }
                Match::PartialMatch => {
                    // Keep searching, but skip this iteration.
                    return None;
                }
                Match::NoMatch => {
                    // We're done searching this map.
                    return None;
                }
            }
        }
        return None;
    }

    /// Insert a mapping from `key` to `value` in the operations map.
    /// Empty `key`s are not allowed.
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
        return Ok(self.insert((key, MappedObject::Op(value))));
    }

    /// Insert a mapping from `key` to `value` in the remap map.
    /// Empty `key`s are not allowed.
    /// Pairs such that `key` and `value` are equal are not allowed.
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
        if key.cmp(&value) == Equal {
            return Err(RemapErr::KeyValueEqual);
        }
        return Ok(self.insert((key, MappedObject::Seq(value))));
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
mod mode_map {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TestOp {
        ThingOne,
        ThingTwo,
    }

    #[test]
    fn insert_overwrite() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );
        assert_eq!(
            Ok(InsertionResult::Overwrite),
            mode_map.insert_op(vec![1u8], TestOp::ThingTwo)
        );
        assert_eq!(
            Ok(InsertionResult::Overwrite),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Overwrite),
            mode_map.insert_remap(vec![1u8], vec![3u8])
        );
    }

    #[test]
    fn insert_empty_key() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Err(OpErr::EmptyKey),
            mode_map.insert_op(Vec::<u8>::new(), TestOp::ThingOne)
        );
        assert_eq!(
            Err(RemapErr::EmptyKey),
            mode_map.insert_remap(Vec::<u8>::new(), vec![1u8])
        );
    }

    #[test]
    fn insert_key_value_parity() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Err(RemapErr::KeyValueEqual),
            mode_map.insert_remap(vec![1u8], vec![1u8])
        );
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
    fn process_overspecified_full_match() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);
        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
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

    // Disabled for now.
    //#[test]
    fn process_shadow_op_with_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        // TODO We need to be able to shadow an op with a remap at some point.
        assert_eq!(
            Ok(InsertionResult::Overwrite),
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

    #[test]
    fn process_remap_then_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
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
        assert_eq!(None, typeahead.pop_front());
    }

    #[test]
    fn process_ambiguous_sequence() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Ambiguous sequence.
        typeahead.push_back(1u8);
        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(1, typeahead.len());
    }

    #[test]
    fn process_disambiguated_remap() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Disambiguate for remap.
        typeahead.push_back(1u8);
        typeahead.push_back(1u8);
        assert_eq!(None, mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(0, typeahead.len());
    }

    #[test]
    fn process_disambiguated_op() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8, 1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_op(vec![1u8], TestOp::ThingOne)
        );

        let mut typeahead = VecDeque::<u8>::new();

        // Disambiguated sequence for op.
        typeahead.push_back(1u8); // Processing just this will result in None.
        typeahead.push_back(2u8); // This one disambiguates, so the op can map.
        assert_eq!(Some(TestOp::ThingOne), mode_map.process(&mut typeahead));
        assert_eq!(Some(2u8), typeahead.pop_front());
        assert_eq!(None, typeahead.pop_front());
    }

    // Test that infinite recursion eventuates in a panic.
    // TODO Instead of panic, consider returning a special InfRecursion op
    // so we can tell the user.
    #[test]
    #[should_panic]
    fn process_inf_recursion() {
        let mut mode_map = ModeMap::<u8, TestOp>::new();
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![1u8], vec![2u8])
        );
        assert_eq!(
            Ok(InsertionResult::Create),
            mode_map.insert_remap(vec![2u8], vec![1u8])
        );
        let mut typeahead = VecDeque::<u8>::new();
        typeahead.push_back(1u8);
        mode_map.process(&mut typeahead);
    }
}
