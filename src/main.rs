extern crate ordered_vec_map;
extern crate termion;

use ordered_vec_map::OrderedVecMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use termion::event::Key;

type KeyMap<T> = OrderedVecMap<Vec<termion::event::Key>, T>;
type KeyRemap = KeyMap<Vec<termion::event::Key>>;

struct Mode<'a, T> {
    typeahead: &'a VecDeque<Key>,
    mode: T,
}

fn put_back(front: &Vec<Key>, typeahead: &mut VecDeque<Key>) {
    for key in front {
        typeahead.push_front(key.clone());
    }
}

struct NormalMode {}

struct InsertMode {}

struct Op {}

impl<'a> Mode<'a, NormalMode> {
    fn new(typeahead: &'a VecDeque<Key>) -> Self {
        Mode {
            typeahead: typeahead,
            mode: NormalMode {},
        }
    }

    fn echo(self, string: &str) {
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
        return std::cmp::Ordering::Equal;
    } else {
        return probe.0.cmp(query);
    };

    // Check for any partial matches against the entire input, where all input
    // keys match the first N map keys.
    map.find_by(partial_matcher).map_or(
        // Then, check for full matches. If any are found, return the longest
        // full match, where all map keys match the first N input keys.
        // Otherwise, return no match.
        map.find(query).map_or(Match::NoMatch, |val| {
            Match::FullMatch(val)
        }),
        |val| Match::PartialMatch(val),
    )
}

struct ModeMap {
    // TODO Handle noremap (key,value) by surrounding value with non-input-able
    // keys, so if it gets put in the typeahead, it cannot possibly be remapped.
    // This would also mean such values would be ignored by the op-map.
    key_remap: KeyRemap,
    op_map: KeyMap<Op>,
}

impl ModeMap {
    // Loop until a partly matching mapping is found or all (local) mappings
    // have been checked.  The longest full match is remembered in "mp_match".
    // A full match is only accepted if there is no partly match, so "aa" and
    // "aaa" can both be mapped.
    // https://github.com/vim/vim/blob/master/src/getchar.c#L2140-L2146
    fn process(self, typeahead: &mut VecDeque<Key>) {
        // Grab incrementally more keys from the front of the queue, looking for
        // matches.
        let mut front = Vec::<Key>::with_capacity(typeahead.len());
        while !typeahead.is_empty() {
            front.push(typeahead.pop_front().unwrap());
            match find_match(&self.key_remap, &front) {
                Match::FullMatch(mapped_keys) => {
                    // Put mapped keys in front of the typeahead buffer.
                    put_back(mapped_keys, typeahead);
                    // Clear front (ergo, we'll skip matching noremap and op
                    // until next iteration).
                    front.clear();
                }
                Match::PartialMatch(_) => {
                    // Keep searching.
                }
                Match::NoMatch => {
                    // We're done searching.
                    return;
                }
            };
            match find_match(&self.op_map, &front) {
                Match::FullMatch(op) => {
                    // TODO Apply op.
                    // op()

                    // Clear front (ergo, we'll skip matching noremap and op
                    // until next iteration).
                    front.clear();
                }
                Match::PartialMatch(_) => {
                    // Keep searching.
                }
                Match::NoMatch => {
                    // We're done searching.
                    return;
                }
            };
        }
        // Put whatever is left back in the typeahead buffer.
        put_back(&front, typeahead);
    }
}

fn main() {
    let mut typeahead = VecDeque::<Key>::new();
    typeahead.push_back(Key::Char('i'));
    typeahead.push_back(Key::Esc);

    let normal_mode = Mode::<NormalMode>::new(&typeahead);
    let insert_mode = Mode::<InsertMode>::from(normal_mode);
    let normal_mode = Mode::<NormalMode>::from(insert_mode);
    normal_mode.echo("asdf");
}
