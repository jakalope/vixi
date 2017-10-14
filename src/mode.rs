extern crate termion;

use std::cmp::Ord;
use std::cmp::Ordering::Equal;
use std::collections::VecDeque;
use common::{prepend, move_to_front};
use ordered_vec_map::OrderedVecMap;
use termion::event::Key;

type KeyMap<T> = OrderedVecMap<Vec<termion::event::Key>, T>;
type KeyRemap = KeyMap<Vec<termion::event::Key>>;

pub struct Mode<'a, T> {
    typeahead: &'a VecDeque<Key>,
    mode: T,
}

pub struct NormalMode {}

pub struct InsertMode {}

pub struct Op {}

// TODO use an enum for modes then pattern match the return type of each mode's
// process() method, which can return any mode but modes only implement From
// a given mode type when it's appropriate!

impl<'a> Mode<'a, NormalMode> {
    pub fn new(typeahead: &'a VecDeque<Key>) -> Self {
        Mode {
            typeahead: typeahead,
            mode: NormalMode {},
        }
    }

    pub fn echo(self, string: &str) {
        println!("{}", string)
    }
}

impl<'a> From<Mode<'a, InsertMode>> for Mode<'a, NormalMode> {
    fn from(current: Mode<'a, InsertMode>) -> Mode<'a, NormalMode> {
        Mode {
            typeahead: current.typeahead,
            mode: NormalMode {},
        }
    }
}

impl<'a> From<Mode<'a, NormalMode>> for Mode<'a, InsertMode> {
    fn from(current: Mode<'a, NormalMode>) -> Mode<'a, InsertMode> {
        Mode {
            typeahead: current.typeahead,
            mode: InsertMode {},
        }
    }
}

enum Match<T> {
    FullMatch(T),
    PartialMatch(T),
    NoMatch,
}

fn find_match<'a, T>(map: &'a KeyMap<T>, query: &Vec<Key>) -> Match<&'a T> {
    let partial_matcher = |probe: &(Vec<Key>, T)| if probe.0.len() >
        query.len() &&
        probe.0.starts_with(query)
    {
        return Equal;
    } else {
        return probe.0.cmp(query);
    };

    // Check for any partial matches against the entire input, where all input
    // keys match the first N map keys.
    map.find_by(partial_matcher).map_or(
        // Then, check for full matches. If any are found, return the longest
        // full match, where all map keys match the first N input keys.
        // Otherwise, return no match.
        map.find(query).map_or(Match::NoMatch, |full| {
            Match::FullMatch(full)
        }),
        |partial| Match::PartialMatch(partial),
    )
}

pub struct ModeMap {
    // TODO Handle noremap (key,value) by surrounding value with non-input-able
    // keys, so if it gets put in the typeahead, it cannot possibly be remapped.
    // This would also mean such values would be ignored by the op-map.
    key_remap: KeyRemap,
    op_map: KeyMap<Op>,
}

fn remap(
    map: &KeyRemap,
    front: &mut Vec<Key>,
    typeahead: &mut VecDeque<Key>,
) -> bool {
    match find_match(map, front) {
        Match::FullMatch(mapped_keys) => {
            // Put mapped keys in front of the typeahead buffer.
            prepend(mapped_keys, typeahead);
            // Clear front (ergo, we'll skip matching noremap and op
            // until next iteration).
            front.clear();
            false
        }
        Match::PartialMatch(_) => {
            // Keep searching.
            false
        }
        Match::NoMatch => {
            // We're done searching this map.
            true
        }
    }
}

fn do_op(
    map: &KeyMap<Op>,
    front: &mut Vec<Key>,
    typeahead: &mut VecDeque<Key>,
) -> bool {
    match find_match(map, front) {
        Match::FullMatch(op) => {
            // TODO Apply op.
            // op()

            // Clear front (ergo, we'll skip matching noremap and op
            // until next iteration).
            front.clear();
            false
        }
        Match::PartialMatch(_) => {
            // Keep searching.
            false
        }
        Match::NoMatch => {
            // We're done searching this map.
            true
        }
    }
}

impl ModeMap {
    // Loop until a partly matching mapping is found or all (local) mappings
    // have been checked.  The longest full match is remembered in "mp_match".
    // A full match is only accepted if there is no partly match, so "aa" and
    // "aaa" can both be mapped.
    // https://github.com/vim/vim/blob/master/src/getchar.c#L2140-L2146
    pub fn process(&self, mut typeahead: &mut VecDeque<Key>) {
        // Grab incrementally more keys from the front of the queue, looking for
        // matches.
        let mut front = Vec::<Key>::with_capacity(typeahead.len());
        let mut remap_done = false;
        let mut op_done = false;
        while !typeahead.is_empty() && (!remap_done || !op_done) {
            front.push(typeahead.pop_front().unwrap());
            remap_done = remap_done ||
                remap(&self.key_remap, &mut front, &mut typeahead);
            op_done = op_done ||
                do_op(&self.op_map, &mut front, &mut typeahead);
        }
        // Put whatever is left back in the typeahead buffer.
        move_to_front(&mut front, typeahead);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_from_normal() {
        let typeahead = VecDeque::<Key>::new();
        let normal_mode = Mode::<NormalMode>::new(&typeahead);
        let _ = Mode::<InsertMode>::from(normal_mode);
    }

    #[test]
    fn test_normal_from_insert() {
        let typeahead = VecDeque::<Key>::new();
        let normal_mode = Mode::<NormalMode>::new(&typeahead);
        let insert_mode = Mode::<InsertMode>::from(normal_mode);
        let _ = Mode::<NormalMode>::from(insert_mode);
    }
}