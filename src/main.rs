use std::collections::VecDeque;
use std::collections::HashMap;

#[allow(dead_code)]
struct Mode<'a, T> {
    typeahead_buffer: &'a VecDeque<u32>,
    mode: T
}

struct NormalMode {
}

struct InsertMode {
}

struct TypeaheadMap {
    typeahead_map: HashMap<Vec<u32>, Vec<u32>>,
}

impl<'a> Mode<'a, NormalMode> {
    fn new(typeahead_buffer: &'a VecDeque<u32>) -> Self {
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
    let typeahead_buffer = VecDeque::<u32>::new();
    let normal_mode = Mode::<NormalMode>::new(&typeahead_buffer);

    // let test_seq = { 
    let insert_mode = Mode::<InsertMode>::from(normal_mode);
    let normal_mode = Mode::<NormalMode>::from(insert_mode);
    normal_mode.echo("asdf");
}
