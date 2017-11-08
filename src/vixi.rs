use mode_map::ModeMap;
use ordered_vec_map::InsertionResult;
use op::{HasMotion, HasObject, PendingOp, ObjectOp, MotionOp, InsertOp,
         NormalOp};
use state_machine::StateMachine;
use key::{MultiKey, Key};
use key::parse::parse;

impl HasMotion<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_motion(
        &mut self,
        key: Vec<MultiKey>,
        op: MotionOp,
    ) -> InsertionResult {
        self.insert_op(key, NormalOp::Motion(op))
    }
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

fn add_motions<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasMotion<MultiKey>,
{
    map.insert_motion(parse("h"), MotionOp::Left);
    map.insert_motion(parse("l"), MotionOp::Right);
    map.insert_motion(parse("k"), MotionOp::Up);
    map.insert_motion(parse("j"), MotionOp::Down);
    map.insert_motion(parse("gg"), MotionOp::Top);
    map.insert_motion(parse("G"), MotionOp::Bottom);
    map.insert_motion(parse("w"), MotionOp::Word);
}

fn add_objects<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasObject<MultiKey>,
{
    map.insert_object(parse("aw"), ObjectOp::AWord);
    map.insert_object(parse("iw"), ObjectOp::InnerWord);
    map.insert_object(parse("aW"), ObjectOp::AWORD);
    map.insert_object(parse("iW"), ObjectOp::InnerWORD);
    map.insert_object(parse("as"), ObjectOp::ASentence);
    map.insert_object(parse("is"), ObjectOp::InnerSentence);
    map.insert_object(parse("ap"), ObjectOp::AParagraph);
    map.insert_object(parse("ip"), ObjectOp::InnerParagraph);
    map.insert_object(parse("a["), ObjectOp::ASquareBlock);
    map.insert_object(parse("a]"), ObjectOp::ASquareBlock);
    map.insert_object(parse("i["), ObjectOp::InnerSquareBlock);
    map.insert_object(parse("i]"), ObjectOp::InnerSquareBlock);
    map.insert_object(parse("a("), ObjectOp::AParen);
    map.insert_object(parse("a)"), ObjectOp::AParen);
    map.insert_object(parse("i("), ObjectOp::InnerParen);
    map.insert_object(parse("i)"), ObjectOp::InnerParen);
    map.insert_object(parse("a<"), ObjectOp::AAngle);
    map.insert_object(parse("a>"), ObjectOp::AAngle);
    map.insert_object(parse("i<"), ObjectOp::InnerAngle);
    map.insert_object(parse("i>"), ObjectOp::InnerAngle);
    map.insert_object(parse("at"), ObjectOp::ATag);
    map.insert_object(parse("it"), ObjectOp::InnerTag);
    map.insert_object(parse("a{"), ObjectOp::ABrace);
    map.insert_object(parse("a}"), ObjectOp::ABrace);
    map.insert_object(parse("i{"), ObjectOp::InnerBrace);
    map.insert_object(parse("i}"), ObjectOp::InnerBrace);
    map.insert_object(parse("a'"), ObjectOp::ASingleQuote);
    map.insert_object(parse("i'"), ObjectOp::InnerSingleQuote);
    map.insert_object(parse("a\""), ObjectOp::ADoubleQuote);
    map.insert_object(parse("i\""), ObjectOp::InnerDoubleQuote);
    map.insert_object(parse("a`"), ObjectOp::ABackTick);
    map.insert_object(parse("i`"), ObjectOp::InnerBackTick);
}

fn normal_mode_map() -> ModeMap<MultiKey, NormalOp> {
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), NormalOp::Cancel);
    map.insert_op(parse("i"), NormalOp::Insert);
    map.insert_op(parse("d"), NormalOp::Delete);
    map.insert_op(parse("."), NormalOp::Delete);
    add_motions(&mut map);
    return map;
}

fn pending_mode_map() -> ModeMap<MultiKey, PendingOp> {
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), PendingOp::Cancel);
    add_objects(&mut map);
    add_motions(&mut map);
    return map;
}

fn insert_mode_map() -> ModeMap<MultiKey, InsertOp> {
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), InsertOp::Cancel);
    return map;
}
