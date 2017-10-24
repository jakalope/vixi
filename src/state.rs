use mode_map::ModeMap;
use op::{NormalOp, InsertOp};
use typeahead::{RemapType, Typeahead};
use termion::event::Key;

fn str_to_keyvec(s: &str) -> Vec<Key> {
    let mut v = Vec::<Key>::with_capacity(s.len());
    for c in s.chars() {
        v.push(Key::Char(c));
    }
    return v;
}

fn make_normal_mode_map() -> ModeMap<Key, NormalOp> {
    let mut map = ModeMap::new();
    map.insert_op(str_to_keyvec("i"), NormalOp::Insert);
    map.insert_op(str_to_keyvec("d"), NormalOp::Delete);
    return map;
}

fn make_insert_mode_map() -> ModeMap<Key, InsertOp> {
    let mut map = ModeMap::new();
    map.insert_op(vec![Key::Esc], InsertOp::Cancel);
    return map;
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub typeahead: Typeahead<Key>,
    pub normal_mode_map: ModeMap<Key, NormalOp>,
    pub insert_mode_map: ModeMap<Key, InsertOp>,
}

impl State {
    pub fn new() -> Self {
        State {
            typeahead: Typeahead::<Key>::new(),
            normal_mode_map: make_normal_mode_map(),
            insert_mode_map: make_insert_mode_map(),
        }
    }

    pub fn put(&mut self, key: Key, remap_type: RemapType) {
        self.typeahead.push_back(key, remap_type);
    }
}
