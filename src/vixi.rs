use serde_json::from_str;
use termion::event::{Event, Key, parse_event};

use mode_map::ModeMap;
use ordered_vec_map::InsertionResult;
use op::{HasMotion, HasObject, PendingOp, ObjectOp, MotionOp, InsertOp,
         NormalOp};
use state_machine::StateMachine;

fn keys_from_str(s: &'static str) -> Vec<Key> {
    let mut v = Vec::new();
    let ref mut bytes = s.bytes().map(|x| Ok(x));
    match bytes.next().unwrap() {
        Ok(b) => {
            match parse_event(b, bytes).unwrap() {
                Event::Key(key) => v.push(key),
                _ => {}
            }
        }
        Err(_) => {}
    }
    return v;
}

macro_rules! add_motion {
    ($map:ident, $name:tt, $op:expr) => {
        $map.insert_motion(keys_from_str($name), $op);
    }
}

macro_rules! add_motions {
    ($map:ident) => {
        add_motion!($map, "h", MotionOp::Left);
        add_motion!($map, "l", MotionOp::Right);
        add_motion!($map, "k", MotionOp::Up);
        add_motion!($map, "j", MotionOp::Down);
        add_motion!($map, "gg", MotionOp::Top);
        add_motion!($map, "G",  MotionOp::Bottom);
        add_motion!($map, "w",  MotionOp::Word);
    }
}

macro_rules! add_object {
    ($map:ident, $name:tt, $op:expr) => {
        $map.insert_object(keys_from_str($name), $op);
    }
}

macro_rules! add_objects {
    ($map:ident) => {
        add_object!($map, "aw", ObjectOp::AWord);
        add_object!($map, "iw", ObjectOp::InnerWord);
        add_object!($map, "aW", ObjectOp::AWORD);
        add_object!($map, "iW", ObjectOp::InnerWORD);
    }
}

impl<K> HasMotion<K> for ModeMap<K, NormalOp>
where
    K: Ord,
    K: Copy,
{
    fn insert_motion(&mut self, key: Vec<K>, op: MotionOp) -> InsertionResult {
        self.insert_op(key, NormalOp::Motion(op))
    }
}

fn normal_mode_map() -> ModeMap<Key, NormalOp> {
    let mut map = ModeMap::<Key, NormalOp>::new();
    map.insert_op(keys_from_str("i"), NormalOp::Insert);
    map.insert_op(keys_from_str("d"), NormalOp::Delete);
    add_motions!(map);
    return map;
}

impl<K> HasMotion<K> for ModeMap<K, PendingOp>
where
    K: Ord,
    K: Copy,
{
    fn insert_motion(&mut self, key: Vec<K>, op: MotionOp) -> InsertionResult {
        self.insert_op(key, PendingOp::Motion(op))
    }
}

impl<K> HasObject<K> for ModeMap<K, PendingOp>
where
    K: Ord,
    K: Copy,
{
    fn insert_object(&mut self, key: Vec<K>, op: ObjectOp) -> InsertionResult {
        self.insert_op(key, PendingOp::Object(op))
    }
}

fn pending_mode_map() -> ModeMap<Key, PendingOp> {
    let mut map = ModeMap::<Key, PendingOp>::new();
    add_motions!(map);
    add_objects!(map);
    return map;
}
