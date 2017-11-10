use mode_map::ModeMap;
use ordered_vec_map::InsertionResult;
use op::{HasOperator, HasMotion, HasObject, PendingOp, ObjectOp, MotionOp,
         InsertOp, NormalOp, OperatorOp};
use state_machine::StateMachine;
use key::{MultiKey, Key};
use key::parse::parse;

impl HasOperator<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_operator(
        &mut self,
        key: Vec<MultiKey>,
        op: OperatorOp,
    ) -> InsertionResult {
        self.insert_op(key, NormalOp::Operator(op))
    }
}

impl HasMotion<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_motion(
        &mut self,
        key: Vec<MultiKey>,
        op: MotionOp,
    ) -> InsertionResult {
        self.insert_op(key, NormalOp::Motion(op))
    }
}

impl HasOperator<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_operator(
        &mut self,
        key: Vec<MultiKey>,
        op: OperatorOp,
    ) -> InsertionResult {
        self.insert_op(key, PendingOp::Operator(op))
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

fn add_operators<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasOperator<MultiKey>,
{
    use op::OperatorOp::*;
    map.insert_operator(parse("c"), Change);
    map.insert_operator(parse("d"), Delete);
    map.insert_operator(parse("y"), Yank);
    map.insert_operator(parse("g~"), SwapCase);
    map.insert_operator(parse("gu"), ToLower);
    map.insert_operator(parse("gU"), ToUpper);
    map.insert_operator(parse("!"), ExternalPrg);
    map.insert_operator(parse("="), EqualPrg);
    map.insert_operator(parse("gq"), TextFormat);
    map.insert_operator(parse("g?"), Rot13);
    map.insert_operator(parse(">"), ShiftRight);
    map.insert_operator(parse("<"), ShiftLeft);
    map.insert_operator(parse("zf"), DefineFold);
    map.insert_operator(parse("g@"), OperatorFunc);
}

fn add_motions<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasMotion<MultiKey>,
{
    use op::MotionOp::*;
    map.insert_motion(parse("h"), Left);
    map.insert_motion(parse("l"), Right);
    map.insert_motion(parse("k"), Up);
    map.insert_motion(parse("j"), Down);
    map.insert_motion(parse("gg"), Top);
    map.insert_motion(parse("G"), Bottom);
    map.insert_motion(parse("w"), Word);
}

fn add_objects<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasObject<MultiKey>,
{
    use op::ObjectOp::*;
    map.insert_object(parse("aw"), AWord);
    map.insert_object(parse("iw"), InnerWord);
    map.insert_object(parse("aW"), AWORD);
    map.insert_object(parse("iW"), InnerWORD);
    map.insert_object(parse("as"), ASentence);
    map.insert_object(parse("is"), InnerSentence);
    map.insert_object(parse("ap"), AParagraph);
    map.insert_object(parse("ip"), InnerParagraph);
    map.insert_object(parse("a["), ASquareBlock);
    map.insert_object(parse("a]"), ASquareBlock);
    map.insert_object(parse("i["), InnerSquareBlock);
    map.insert_object(parse("i]"), InnerSquareBlock);
    map.insert_object(parse("a("), AParen);
    map.insert_object(parse("a)"), AParen);
    map.insert_object(parse("i("), InnerParen);
    map.insert_object(parse("i)"), InnerParen);
    map.insert_object(parse("a<"), AAngle);
    map.insert_object(parse("a>"), AAngle);
    map.insert_object(parse("i<"), InnerAngle);
    map.insert_object(parse("i>"), InnerAngle);
    map.insert_object(parse("at"), ATag);
    map.insert_object(parse("it"), InnerTag);
    map.insert_object(parse("a{"), ABrace);
    map.insert_object(parse("a}"), ABrace);
    map.insert_object(parse("i{"), InnerBrace);
    map.insert_object(parse("i}"), InnerBrace);
    map.insert_object(parse("a'"), ASingleQuote);
    map.insert_object(parse("i'"), InnerSingleQuote);
    map.insert_object(parse("a\""), ADoubleQuote);
    map.insert_object(parse("i\""), InnerDoubleQuote);
    map.insert_object(parse("a`"), ABackTick);
    map.insert_object(parse("i`"), InnerBackTick);
}

fn normal_mode_map() -> ModeMap<MultiKey, NormalOp> {
    use op::NormalOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    map.insert_op(parse("i"), Insert);
    map.insert_op(parse("."), Repeat);
    add_operators(&mut map);
    add_motions(&mut map);
    return map;
}

fn pending_mode_map() -> ModeMap<MultiKey, PendingOp> {
    use op::PendingOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    add_operators(&mut map);
    add_objects(&mut map);
    add_motions(&mut map);
    return map;
}

fn insert_mode_map() -> ModeMap<MultiKey, InsertOp> {
    use op::InsertOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    return map;
}

pub struct Vixi {
    mode: StateMachine<MultiKey>,
}

impl Vixi {
    pub fn new() -> Self {
        Vixi {
            mode: StateMachine::with_maps(
                normal_mode_map(),
                pending_mode_map(),
                insert_mode_map(),
            ),
        }
    }

    pub fn process(&mut self, keys: &str) {
        for key in parse(keys) {
            self.mode.process(key);
        }
    }

    pub fn mode(&self) -> &'static str {
        self.mode.mode()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_in_normal() {
        let mut vixi = Vixi::new();
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_insert() {
        let mut vixi = Vixi::new();
        vixi.process("i");
        assert_eq!("Insert", vixi.mode());
    }

    #[test]
    fn insert_to_normal() {
        let mut vixi = Vixi::new();
        vixi.process("i<esc>");
        assert_eq!("Normal", vixi.mode());
    }

    #[test]
    fn to_op_pending() {
        let mut vixi = Vixi::new();
        vixi.process("d");
        assert_eq!("Pending", vixi.mode());
    }

    #[test]
    fn op_pending_to_normal() {
        let mut vixi = Vixi::new();
        vixi.process("d<esc>");
        assert_eq!("Normal", vixi.mode());
    }
}
