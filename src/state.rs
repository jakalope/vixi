use mode_map::ModeMap;
use op::{NormalOp, InsertOp};
use typeahead::Typeahead;
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

pub struct State<K>
where
    K: Ord,
    K: Copy,
{
    pub typeahead: Typeahead<K>,
    pub normal_mode_map: ModeMap<K, NormalOp>,
    pub insert_mode_map: ModeMap<K, InsertOp>,
}

impl<K> State<K>
where
    K: Ord,
    K: Copy,
{
    pub fn new() -> Self {
        State {
            typeahead: Typeahead::<K>::new(),
            normal_mode_map: ModeMap::<K, NormalOp>::new(),
            insert_mode_map: ModeMap::<K, InsertOp>::new(),
        }
    }
}
