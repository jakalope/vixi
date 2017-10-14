use std::fmt::Debug;
use std::cmp::Ord;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::VecDeque;
use common::{prepend, move_to_front};
use ordered_vec_map::OrderedVecMap;

pub enum OpErr {
    NotEnoughArgs(usize),
}

// Function pointer transforming a sequence &Vec<K> to a Result<(), OpErr>.
pub type Op<K> = fn(&Vec<K>) -> Result<(), OpErr>;

// Maps sequence Vec<K> to value of type T.
pub type SeqMap<K, T> = OrderedVecMap<Vec<K>, T>;

// Maps sequence Vec<K> to sequence Vec<K>.
pub type SeqRemap<K> = OrderedVecMap<Vec<K>, Vec<K>>;

pub struct ModeMap<K>
where
    K: Ord,
    K: Copy,
{
    // TODO Handle noremap (key,value) by surrounding value with non-input-able
    // keys, so if it gets put in the typeahead, it cannot possibly be remapped.
    // This would also mean such values would be ignored by the op-map.
    key_remap: SeqRemap<K>,
    op_map: SeqMap<K, Op<K>>,
}

pub struct State<K>
where
    K: Ord,
    K: Copy,
{
    typeahead: VecDeque<K>,
    insert_mode_map: ModeMap<K>,
    normal_mode_map: ModeMap<K>,
}

impl<K> ModeMap<K>
where
    K: Ord,
    K: Copy,
{
    fn new() -> Self {
        ModeMap::<K> {
            key_remap: SeqRemap::<K>::new(),
            op_map: SeqMap::<K, Op<K>>::new(),
        }
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
            insert_mode_map: ModeMap::<K>::new(),
            normal_mode_map: ModeMap::<K>::new(),
        }
    }
}

enum Match<T> {
    FullMatch(T),
    PartialMatch,
    NoMatch,
}

fn find_match<'a, K, T>(map: &'a SeqMap<K, T>, query: &Vec<K>) -> Match<&'a T>
where
    K: Ord,
    K: Copy,
{
    /// Matches a sequence `Vec<K>` using Vi's mapping rules:
    /// * First check for partial matches such that query is shorter than the
    ///   key. In this case, return `Match::PartialMatch`.
    /// * If no partial match is found, check for a full (exact) match. In this
    ///   case, return the value it maps to in a `Match::FullMatch` variant.
    /// * If no partial or full match is found, return `Match::NoMatch`.
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

    map.find_by(partial_matcher).map_or(
        map.find(query).map_or(
            Match::NoMatch, // No partial or full match found.
            |full| Match::FullMatch(full), // No partial match, found full.
        ),
        |_| Match::PartialMatch, // Found a partial match.
    )
}


fn remap<K>(
    map: &SeqRemap<K>,
    front: &mut Vec<K>,
    typeahead: &mut VecDeque<K>,
) -> bool
where
    K: Ord,
    K: Copy,
{
    match find_match(map, front) {
        Match::FullMatch(mapped_keys) => {
            // Put mapped keys in front of the typeahead buffer.
            prepend(mapped_keys, typeahead);
            // Clear front (ergo, we'll skip matching noremap and op until next
            // iteration).
            front.clear();
            false
        }
        Match::PartialMatch => {
            // Keep searching.
            false
        }
        Match::NoMatch => {
            // We're done searching this map.
            true
        }
    }
}

fn do_op<K>(map: &SeqMap<K, Op<K>>, front: &mut Vec<K>) -> bool
where
    K: Ord,
    K: Copy,
{
    match find_match(map, front) {
        Match::FullMatch(op) => {
            // TODO Apply op.
            // op()

            // Clear front (ergo, we'll skip matching noremap and op
            // until next iteration).
            front.clear();
            false
        }
        Match::PartialMatch => {
            // Keep searching.
            false
        }
        Match::NoMatch => {
            // We're done searching this map.
            true
        }
    }
}

impl<K> ModeMap<K>
where
    K: Ord,
    K: Copy,
{
    // Loop until a partly matching mapping is found or all (local) mappings
    // have been checked.  The longest full match is remembered in "mp_match".
    // A full match is only accepted if there is no partly match, so "aa" and
    // "aaa" can both be mapped.
    // https://github.com/vim/vim/blob/master/src/getchar.c#L2140-L2146
    pub fn process(&self, mut typeahead: &mut VecDeque<K>) {
        // Grab incrementally more keys from the front of the queue, looking for
        // matches.
        let mut front = Vec::<K>::with_capacity(typeahead.len());
        let mut remap_done = false;
        let mut op_done = false;
        while !typeahead.is_empty() && (!remap_done || !op_done) {
            front.push(typeahead.pop_front().unwrap());
            remap_done = remap_done ||
                remap(&self.key_remap, &mut front, &mut typeahead);
            op_done = op_done || do_op(&self.op_map, &mut front);
        }
        // Put whatever is left back in the typeahead buffer.
        move_to_front(&mut front, typeahead);
    }
}

#[cfg(test)]
mod test_find_match {
    use super::*;

    fn assert_partial_match<T>(m: Match<T>) {
        match m {
            Match::PartialMatch => {
                assert!(true);
            }
            Match::FullMatch(_) => {
                assert!(false);
            }
            Match::NoMatch => {
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
            Match::PartialMatch => {
                assert!(false);
            }
            Match::FullMatch(x) => {
                assert_eq!(expected, x);
            }
            Match::NoMatch => {
                assert!(false);
            }
        }
    }

    fn assert_no_match<T>(m: Match<T>) {
        match m {
            Match::PartialMatch => {
                assert!(false);
            }
            Match::FullMatch(x) => {
                assert!(false);
            }
            Match::NoMatch => {
                assert!(true);
            }
        }
    }

    #[test]
    fn test_partial_match() {
        let mut map = SeqMap::<u8, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_partial_match(find_match(&map, &query))
    }

    #[test]
    fn test_full_match() {
        let mut map = SeqMap::<u8, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8], 6u8));
        let query = vec![1u8, 2u8, 3u8];
        assert_full_match(&6u8, find_match(&map, &query))
    }

    #[test]
    fn test_no_match() {
        let mut map = SeqMap::<u8, u8>::new();
        map.insert((vec![1u8, 2u8, 3u8, 4u8], 6u8));
        let query = vec![2u8, 3u8];
        assert_no_match(find_match(&map, &query))
    }
}
