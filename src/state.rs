use common::{prepend, move_to_front};
use op::{NormalOp, InsertOp};
use ordered_vec_map::{InsertionResult, OrderedVecMap};
use std::cmp::min;
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
pub type ModeMap<K, Op> = OrderedVecMap<Vec<K>, MappedObject<K, Op>>;

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

//
// MatchLen   KeyLen     QueryLen  MatchLen   v- MatchLen = min(QueryLen,  KeyLen)
// ----------------------------------------
//          <           <         <             Not possible
//          <           <         =             Not possible
//     1    <     3     <    5    >   1         MatchLen <  KeyLen < QueryLen
//          <           =         <             Not possible
//          <           =         =             Not possible
//     1    <     5     =    5    >   1         MatchLen <  KeyLen = QueryLen
//     3    <     5     >    1    <   3       x QueryLen < MatchLen <  KeyLen
//     3    <     5     >    3    =   3         MatchLen = QueryLen <  KeyLen
//     1    <     5     >    3    >   1         MatchLen < QueryLen <  KeyLen
//          =           <         <             Not possible
//          =           <         =             Not possible
//     1    =     1     <    5    >   1         MatchLen =  KeyLen < QueryLen
//          =           =         <             Not possible
//     3    =     3     =    3    =   3         MatchLen =  KeyLen = QueryLen
//          =           =         >             Not possible
//     5    =     5     >    3    <   5       x QueryLen < MatchLen =  KeyLen
//          =           >         =             Not possible
//          =           >         >             Not possible
//  Not allowed to have == and (< or >).
//  Not allowed to have =  and (<< or >>).
//  Not allowed to have <<<.
//
//  Summary:
//    MatchLen < QueryLen <  KeyLen   =>  Not a match
//    MatchLen <  KeyLen  < QueryLen  =>  Not a match
//    MatchLen <  KeyLen  = QueryLen  =>  Not a match
//    MatchLen = QueryLen <  KeyLen   =>  Partial match |
//    MatchLen =  KeyLen  < QueryLen  =>  Full match    |- MatchLen >=
//    MatchLen =  KeyLen  = QueryLen  =>  Full match    |  min(QueryLen, KeyLen)

fn find_match<'a, K, T>(
    map: &'a OrderedVecMap<Vec<K>, T>,
    query: &Vec<K>,
) -> Match<&'a (Vec<K>, T)>
where
    K: Ord,
{
    let query_len = query.len();
    if query_len == 0 {
        return Match::NoMatch;
    }
    let mut longest_match_key_len: usize = 0;
    let mut longest_match: Option<&(Vec<K>, T)> = None;
    for kv in map.iter() {
        let match_len = match_length(query, &kv.0);
        let key_len = kv.0.len();
        if (match_len >= query_len || match_len >= key_len) &&
            key_len > longest_match_key_len
        {
            longest_match = Some(kv);
            longest_match_key_len = key_len;
        }
    }
    if longest_match_key_len > query_len {
        Match::PartialMatch
    } else if longest_match.is_some() {
        Match::FullMatch(longest_match.unwrap())
    } else {
        Match::NoMatch
    }
}

/// Matches a sequence `Vec<K>` using Vi's mapping rules:
/// * First check for partial matches such that `query` is shorter than the key.
///   In this case, return `Match::PartialMatch`.
/// * If no partial match is found, check for a full match. In this
///   case, return the value it maps to in a `Match::FullMatch` variant.
/// * If no partial or full match is found, return `Match::NoMatch`.
// fn find_match<'a, K, T>(
//     map: &'a OrderedVecMap<Vec<K>, T>,
//     query: &Vec<K>,
// ) -> Match<&'a (Vec<K>, T)>
// where
//     K: Ord,
// {
//     if query.is_empty() {
//         return Match::NoMatch;
//     }

//     let partial_matcher = |probe: &(Vec<K>, T)| match probe.0.cmp(query) {
//         Less => Less,
//         Equal => Less, // When searching for partial matches, ignore equal.
//         Greater => {
//             return if probe.0.len() > query.len() &&
//                 probe.0.starts_with(query)
//             {
//                 Equal
//             } else {
//                 Greater
//             };
//         }
//     };

//     let full_matcher = |probe: &(Vec<K>, T)| match probe.0.cmp(query) {
//         Less => {
//             return if query.starts_with(&probe.0) {
//                 Equal
//             } else {
//                 Less
//             };
//         }
//         Equal => Equal,
//         Greater => Greater,
//     };

//     map.find_by(partial_matcher).map_or(
//         map.find_all_by(full_matcher).map_or(
//             Match::NoMatch, // No partial or full match found.
//             |full| {
//                 Match::FullMatch(full)
//             }, // No partial match, found full.
//         ),
//         |_| Match::PartialMatch, // Found a partial match.
//     )
// }
/// If `query` is a full match to a key found in `map`, the
/// corresponding map value is prepended to the typeahead buffer.
/// Returns `true` if a full or partial match is found. Otherwise, `false`.
// fn remap<K>(
//     map: &OrderedVecMap<Vec<K>, Vec<K>>,
//     query: &mut Vec<K>,
//     typeahead: &mut VecDeque<K>,
// ) -> bool
// where
//     K: Ord,
//     K: Copy,
// {
//     match find_match(map, query) {
//         Match::FullMatch(mapped_keys) => {
//             match mapped_keys {
//                 MappedObject::Seq(seq) => {
//                     let len = mapped_keys.0.len();
//                     typeahead =
//                         mapped_keys.1.splice(..len, query.iter()).collect();
//                 }
//                 MappedObject::Op(op) => Some(op),
//             }
//             true
//         }
//         Match::PartialMatch => {
//             // Keep searching.
//             true
//         }
//         Match::NoMatch => {
//             // We're done searching this map.
//             false
//         }
//     }
// }

fn copy_into_deque<K>(v: &[K], d: &mut VecDeque<K>)
where
    K: Copy,
{
    d.clear();
    for k in v {
        d.push_back(*k);
    }
}

#[cfg(test)]
mod copy_into_deque {
    use super::*;

    #[test]
    fn simple() {
        let v = vec![1u8, 2u8, 3u8];
        let mut d = VecDeque::<u8>::new();
        copy_into_deque(&v, &mut d);
        assert_eq!(Some(1u8), d.pop_front());
        assert_eq!(Some(2u8), d.pop_front());
        assert_eq!(Some(3u8), d.pop_front());
        assert_eq!(None, d.pop_front());
    }
}

fn remap<K>(
    from: &Vec<K>,
    to: Vec<K>,
    mut query: Vec<K>,
    typeahead: &mut VecDeque<K>,
) where
    K: Copy,
{
    let len = from.len();
    query.splice(..len, to);
    copy_into_deque(&query, typeahead);
}

#[cfg(test)]
mod remap {
    use super::*;

    #[test]
    fn simple() {
        let from = vec![1u8];
        let to = vec![2u8];
        let query = vec![1u8, 1u8];
        let mut typeahead = VecDeque::<u8>::new();
        remap(&from, to, query, &mut typeahead);
        assert_eq!(VecDeque::<u8>::from(vec![2u8, 1u8]), typeahead);
    }

    #[test]
    fn one_to_two_three() {
        let from = vec![1u8];
        let to = vec![2u8, 3u8];
        let query = vec![1u8];
        let mut typeahead = VecDeque::<u8>::new();
        remap(&from, to, query, &mut typeahead);
        assert_eq!(VecDeque::<u8>::from(vec![2u8, 3u8]), typeahead);
    }

    #[test]
    fn one_to_two() {
        let from = vec![1u8];
        let to = vec![2u8];
        let query = vec![1u8];
        let mut typeahead = VecDeque::<u8>::new();
        remap(&from, to, query, &mut typeahead);
        assert_eq!(VecDeque::<u8>::from(vec![2u8]), typeahead);
    }

    #[test]
    fn lots_to_nothing() {
        let from = vec![1u8, 2u8, 3u8, 4u8];
        let to = vec![];
        let query = vec![1u8, 2u8, 3u8, 4u8];
        let mut typeahead = VecDeque::<u8>::new();
        remap(&from, to, query, &mut typeahead);
        assert_eq!(VecDeque::<u8>::from(vec![]), typeahead);
    }
}

impl<K, Op> ModeMap<K, Op>
where
    K: Ord,
    K: Copy,
    Op: Copy,
{
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
            // TODO limit query to typeahead.get(..min(longest_key, typeahead.len()))
            let mut query = Vec::<K>::with_capacity(typeahead.len());
            for k in typeahead.iter() {
                query.push(*k);
            }
            match find_match(&self, &query) {
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

// #[cfg(test)]
// mod remap {
//     use super::*;

//     #[test]
//     fn no_match() {
//         let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
//         map.insert((vec![2u8, 3u8], vec![6u8]));
//         let mut query = vec![1u8, 2u8, 3u8];
//         let mut typeahead = VecDeque::<u8>::new();
//         assert_eq!(false, remap(&map, &mut query, &mut typeahead));
//         assert!(typeahead.is_empty());
//     }

//     #[test]
//     fn partial_match() {
//         let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
//         map.insert((vec![1u8, 2u8, 3u8, 4u8], vec![6u8]));
//         let mut query = vec![1u8, 2u8, 3u8];
//         let mut typeahead = VecDeque::<u8>::new();
//         assert_eq!(true, remap(&map, &mut query, &mut typeahead));
//         assert!(typeahead.is_empty());
//     }

//     #[test]
//     fn full_match() {
//         let mut map = OrderedVecMap::<Vec<u8>, Vec<u8>>::new();
//         map.insert((vec![1u8, 2u8, 3u8], vec![6u8]));
//         let mut query = vec![1u8, 2u8, 3u8];
//         let mut typeahead = VecDeque::<u8>::new();
//         assert_eq!(true, remap(&map, &mut query, &mut typeahead));
//         assert_eq!(VecDeque::from(vec![6u8]), typeahead);
//     }
// }

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
        assert_eq!(
            Match::FullMatch(&(vec![1u8, 2u8, 3u8], 6u8)),
            find_match(&map, &query)
        );
    }

    #[test]
    fn best_full_match() {
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
    // #[test]
    // #[should_panic]
    // fn process_inf_recursion() {
    //     let mut mode_map = ModeMap::<u8, TestOp>::new();
    //     assert_eq!(
    //         Ok(InsertionResult::Create),
    //         mode_map.insert_remap(vec![1u8], vec![2u8])
    //     );
    //     assert_eq!(
    //         Ok(InsertionResult::Create),
    //         mode_map.insert_remap(vec![2u8], vec![1u8])
    //     );
    //     let mut typeahead = VecDeque::<u8>::new();
    //     typeahead.push_back(1u8);
    //     mode_map.process(&mut typeahead);
    // }
}
