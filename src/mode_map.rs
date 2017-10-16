use termion::event::{Key, parse_event};
use op::{NormalOp, InsertOp};
use state::ModeMap;

fn make_normal_mode_map() -> ModeMap<Key, NormalOp> {
    let mut map = ModeMap::<Key, NormalOp>::new();
    map.insert_op(vec![Key::Char('i')], NormalOp::Insert);
    return map;
}
