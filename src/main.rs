extern crate termion;

use std::collections::VecDeque;
use std::collections::HashMap;
use termion::event::Key;

#[allow(dead_code)]
struct Mode<'a, T> {
    typeahead_buffer: &'a VecDeque<Key>,
    mode: T
}

struct NormalMode {
}

struct InsertMode {
}

struct Op {
}

// fn execute(op_map: &HashMap<Vec<Key>, Op>,
//            front: &mut Vec<Key>) {
//     match op_map.get(front) {
//         Some(op) => { return true; } // TODO op(),
//         None => { return false; }
//     };
//     // Continue processing what's left in the buffer.
// }

// Vec<> sort_unstable(), binary_search()
// Provides an ordered map with a method to query for partial matches.
// This is useful for disambiguation.
struct OrderedVecMap<KeyT, ValueT> where KeyT: std::cmp::Ord {
    data: Vec<(KeyT, ValueT)>,
}

impl<KeyT, ValueT> OrderedVecMap<KeyT, ValueT> where KeyT: std::cmp::Ord {
    fn new() -> Self {
        OrderedVecMap {
            data: Vec::<(KeyT, ValueT)>::new(),
        }
    }

    // Returns true if the value is inserted, false if overwritten.
    fn insert(&mut self, mut datum: (KeyT, ValueT)) -> bool {
        match self.data.binary_search_by(|probe| probe.0.cmp(&datum.0)) {
            Ok(idx) => {
                *self.data.get_mut(idx).unwrap() = datum;
                return false;
            },
            Err(idx) => {
                self.data.insert(idx, datum);
                return true;
            },
        };
    }

    // On success, returns the value corresponding to the key.
    // On failure, returns Err(true) if the key is fully disambiguated (i.e.,
    // no partial matches exist) or Err(false) if the key has partial matches.
    //
    // Check an entry whether it matches.
    // - Full match: mlen == keylen
    // - Partly match: mlen == tb_len and tb_len < keylen
    // https://github.com/vim/vim/blob/master/src/getchar.c#L2202-L2206
    //
    //   aa
    //   aaa
    //   ab
    //   baa
    //
    // Any partial match: where entire query matches the first N map keys.
    // The longest full match: where entire map matches the first N query keys.
    fn find(&self, query: &KeyT) -> Option<&ValueT> {
        match self.data.binary_search_by(|probe| probe.0.cmp(query)) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(idx) => None,
        }
    }

    fn find_by<'a, F>(&self, f: F) -> Option<&ValueT>
            where F: Fn(&(KeyT, ValueT)) -> std::cmp::Ordering,
                  KeyT: 'a,
                  ValueT: 'a {
        match self.data.binary_search_by(f) {
            Ok(idx) => Some(&self.data.get(idx).unwrap().1),
            Err(_) => None
        }
    }
}

impl<KeyT, ValueT> From<Vec<(KeyT, ValueT)>> for OrderedVecMap<KeyT, ValueT>
        where KeyT: std::cmp::Ord {
    fn from(mut data: Vec<(KeyT, ValueT)>) -> OrderedVecMap<KeyT, ValueT> {
        data.sort_by(|a, b| a.0.cmp(&b.0));
        OrderedVecMap {
            data: data,
        }
    }
}

fn remap(map: &OrderedVecMap<Vec<Key>, Vec<Key>>,
         front: &Vec<Key>,
         typeahead_buffer: &mut VecDeque<Key>) -> bool {
    match map.find(front) {
        Some(mapped_keys) => {
            // Clone mapped keys in front of the typeahead buffer.
            for key in mapped_keys {
                typeahead_buffer.push_front(key.clone());
            }
            return true;
        },
        None => { return false; }
    };
}

enum Match<T> {
    FullMatch(T),
    PartialMatch(T),
    NoMatch
}

// Check for any partial matches against the entire input, where all input
// keys match the first N map keys.
// Then, check for full matches. If any are found, return the longest full
// match, where all map keys match the first N input keys.
fn find_match<'a>(map: &'a OrderedVecMap<Vec<Key>, Vec<Key>>,
                  query: &Vec<Key>) -> Match<&'a Vec<Key>> {
    let partial_matcher = |probe: &(Vec<Key>, Vec<Key>)| {
            if probe.0.len() > query.len() && probe.0.starts_with(query) {
                return std::cmp::Ordering::Equal;
            } else {
                return probe.0.cmp(query);
            }
        };

    map.find_by(partial_matcher).map_or(
            map.find(query).map_or(Match::NoMatch, |val| Match::FullMatch(val)),
            |val| Match::PartialMatch(val)
        )
}
        
// struct ModeMap {
//     key_remap: OrderedVecMap<Vec<Key>, Vec<Key>>,
//     key_noremap: OrderedVecMap<Vec<Key>, Vec<Key>>,
//     op_map: HashMap<Vec<Key>, Op>,
// }

// impl ModeMap {
//     // Loop until a partly matching mapping is found or all (local) mappings
//     // have been checked.  The longest full match is remembered in "mp_match".
//     // A full match is only accepted if there is no partly match, so "aa" and
//     // "aaa" can both be mapped.
//     // https://github.com/vim/vim/blob/master/src/getchar.c#L2140-L2146
//     fn process(self, typeahead_buffer: &mut VecDeque<Key>) {
//         // Grab incrementally more keys from the front of the
//         // queue, looking for matches.
//         let mut front = VecDeque::<Key>::with_capacity(typeahead_buffer.len());
//         while !typeahead_buffer.is_empty() {
//             front.push_back(typeahead_buffer.pop_front().unwrap());
//             if remap(&self.key_remap, &mut front, typeahead_buffer) {
//                 // Tail recursion on successful remap.
//                 return self.process(typeahead_buffer);
//             }
//             remap(&self.key_noremap, &mut front, typeahead_buffer);
//         }
//         // TODO awaiting a full mapping isn't the same as awaiting a
//         // disambiguating key.
//         if self.op_map.contains_key(&mut front) {
//             // Drop keys in front.
//             front.clear();
//             // Apply operation.
//         }

//         // Put whatever is left back in the typeahead buffer.
//         let typeahead_buffer = front;
//     }
// }

impl<'a> Mode<'a, NormalMode> {
    fn new(typeahead_buffer: &'a VecDeque<Key>) -> Self {
        Mode {
            typeahead_buffer: typeahead_buffer,
            mode: NormalMode { },
        }
    }

    fn echo(self, string: &str) {
        println!("{}", string)
    }
}

impl<'a> From<Mode<'a, InsertMode>> for Mode<'a, NormalMode> {
    fn from(current: Mode<'a, InsertMode>) -> Mode<'a, NormalMode> {
        Mode {
            typeahead_buffer: current.typeahead_buffer,
            mode: NormalMode { },
        }
    }
}

impl<'a> From<Mode<'a, NormalMode>> for Mode<'a, InsertMode> {
    fn from(current: Mode<'a, NormalMode>) -> Mode<'a, InsertMode> {
        Mode {
            typeahead_buffer: current.typeahead_buffer,
            mode: InsertMode { },
        }
    }
}

fn main() {
    let mut typeahead_buffer = VecDeque::<Key>::new();
    typeahead_buffer.push_back(Key::Char('i'));
    typeahead_buffer.push_back(Key::Esc);

    let normal_mode = Mode::<NormalMode>::new(&typeahead_buffer);

    let insert_mode = Mode::<InsertMode>::from(normal_mode);
    let normal_mode = Mode::<NormalMode>::from(insert_mode);
    normal_mode.echo("asdf");
}
