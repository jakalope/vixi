use mode_map::ModeMap;
use ordered_vec_map::InsertionResult;
use op::{HasMotion, HasObject, PendingOp, ObjectOp, MotionOp, InsertOp,
         NormalOp};
use state_machine::StateMachine;
use key::{parse, MultiKey, Key};

impl HasMotion<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_motion(
        &mut self,
        key: Vec<MultiKey>,
        op: MotionOp,
    ) -> InsertionResult {
        self.insert_op(key, NormalOp::Motion(op))
    }
}

fn add_motions<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasMotion<MultiKey>,
{
    map.insert_motion(parse::parse("h"), MotionOp::Left);
    map.insert_motion(parse::parse("l"), MotionOp::Right);
    map.insert_motion(parse::parse("k"), MotionOp::Up);
    map.insert_motion(parse::parse("j"), MotionOp::Down);
    map.insert_motion(parse::parse("gg"), MotionOp::Top);
    map.insert_motion(parse::parse("G"), MotionOp::Bottom);
    map.insert_motion(parse::parse("w"), MotionOp::Word);
}

macro_rules! add_object {
    ($map:ident, $name:tt, $op:expr) => {
        $map.insert_object(parse::parse($name), $op);
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

fn normal_mode_map() -> ModeMap<MultiKey, NormalOp> {
    let mut map = ModeMap::<MultiKey, NormalOp>::new();
    map.insert_op(parse::parse("i"), NormalOp::Insert);
    map.insert_op(parse::parse("d"), NormalOp::Delete);
    add_motions(&mut map);
    return map;
}

impl HasMotion<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_motion(
        &mut self,
        key: Vec<MultiKey>,
        op: MotionOp,
    ) -> InsertionResult {
        self.insert_op(key, PendingOp::Motion(op))
    }
}

impl HasObject<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_object(
        &mut self,
        key: Vec<MultiKey>,
        op: ObjectOp,
    ) -> InsertionResult {
        self.insert_op(key, PendingOp::Object(op))
    }
}

fn pending_mode_map() -> ModeMap<MultiKey, PendingOp> {
    let mut map = ModeMap::<MultiKey, PendingOp>::new();
    add_objects!(map);
    add_motions(&mut map);
    return map;
}
