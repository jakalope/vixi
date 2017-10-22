use termion::event::{Key, parse_event};
use op::{NormalOp, InsertOp};
use state::ModeMap;

fn str_to_keyvec(s: &str) -> Vec<Key> {
    let mut v = Vec::<Key>::with_capacity(s.len());
    for c in s.chars() {
        v.push(Key::Char(c));
    }
    return v;
}

fn make_normal_mode_map() -> ModeMap<Key, NormalOp> {
    let mut map = ModeMap::new();
    map.insert_op(str_to_keyvec("i"), NormalOp::Insert).unwrap();
    map.insert_op(str_to_keyvec("d"), NormalOp::Delete).unwrap();
    return map;
}
