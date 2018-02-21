use key::parse::parse;
use key::MultiKey;
use mode_map::ModeMap;
use op::{HasOperator, HasMotion, HasObject, PendingOp, ObjectOp, MotionOp, InsertOp, NormalOp,
         OperatorOp};
use ordered_vec_map::InsertionResult;

impl HasOperator<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_operator(&mut self, key: Vec<MultiKey>, op: OperatorOp) -> InsertionResult {
        self.insert_op(key, NormalOp::Operator(op))
    }
}

impl HasMotion<MultiKey> for ModeMap<MultiKey, NormalOp> {
    fn insert_motion(&mut self, key: Vec<MultiKey>, op: MotionOp) -> InsertionResult {
        self.insert_op(key, NormalOp::Motion(op))
    }
}

impl HasOperator<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_operator(&mut self, key: Vec<MultiKey>, op: OperatorOp) -> InsertionResult {
        self.insert_op(key, PendingOp::Operator(op))
    }
}

impl HasMotion<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_motion(&mut self, key: Vec<MultiKey>, op: MotionOp) -> InsertionResult {
        self.insert_op(key, PendingOp::Motion(op))
    }
}

impl HasObject<MultiKey> for ModeMap<MultiKey, PendingOp> {
    fn insert_object(&mut self, key: Vec<MultiKey>, op: ObjectOp) -> InsertionResult {
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
    map.insert_operator(parse("<lt>"), ShiftLeft);
    map.insert_operator(parse("zf"), DefineFold);
    map.insert_operator(parse("g@"), OperatorFunc);
}

fn add_motions<Op>(map: &mut ModeMap<MultiKey, Op>)
where
    Op: Copy,
    ModeMap<MultiKey, Op>: HasMotion<MultiKey>,
{
    use op::MotionOp::*;
    map.insert_motion(parse("<left>"), Left);
    map.insert_motion(parse("<right>"), Right);
    map.insert_motion(parse("<up>"), Up);
    map.insert_motion(parse("<down>"), Down);
    map.insert_motion(parse("h"), Left);
    map.insert_motion(parse("l"), Right);
    map.insert_motion(parse("k"), Up);
    map.insert_motion(parse("j"), Down);
    map.insert_motion(parse("gg"), Top);
    map.insert_motion(parse("G"), Bottom);
    map.insert_motion(parse("w"), Word);
    map.insert_motion(parse("<bs>"), Backspace);
    map.insert_motion(parse("<home>"), Home);
    map.insert_motion(parse("<end>"), End);
    map.insert_motion(parse("<khome>"), Home);
    map.insert_motion(parse("<kend>"), End);
    map.insert_motion(parse("<pageup>"), PageUp);
    map.insert_motion(parse("<pagedown>"), PageDown);
    map.insert_motion(parse("<kpageUp>"), PageUp);
    map.insert_motion(parse("<kpageDown>"), PageDown);
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

pub fn normal_mode_map() -> ModeMap<MultiKey, NormalOp> {
    use op::NormalOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    map.insert_op(parse("i"), Insert);
    map.insert_op(parse("r"), ReplaceChar);
    map.insert_op(parse("R"), ReplaceMode);
    map.insert_op(parse("."), Repeat);
    add_operators(&mut map);
    add_motions(&mut map);
    return map;
}

pub fn pending_mode_map() -> ModeMap<MultiKey, PendingOp> {
    use op::PendingOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    add_operators(&mut map);
    add_objects(&mut map);
    add_motions(&mut map);
    return map;
}

pub fn insert_mode_map() -> ModeMap<MultiKey, InsertOp> {
    use op::InsertOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    map.insert_op(parse("<Up>"), Up);
    map.insert_op(parse("<Down>"), Down);
    map.insert_op(parse("<Left>"), Left);
    map.insert_op(parse("<Right>"), Right);
    map.insert_op(parse("<Backspace>"), Backspace);
    map.insert_op(parse("<Delete>"), Delete);
    map.insert_op(parse("<PageUp>"), PageUp);
    map.insert_op(parse("<PageDown>"), PageDown);
    map.insert_op(parse("<kpageUp>"), PageUp);
    map.insert_op(parse("<kpageDown>"), PageDown);
    map.insert_op(parse("<home>"), Home);
    map.insert_op(parse("<end>"), End);
    map.insert_op(parse("<khome>"), Home);
    map.insert_op(parse("<kend>"), End);
    map.insert_op(parse("<C-w>"), DeleteWord); // (Ctrl-W));
    map.insert_op(parse("<C-u>"), DeleteLine); // (Ctrl-U));
    map.insert_op(parse("<Tab>"), Tab);
    map.insert_op(parse("<C-k>"), Digraph); // (Ctrl-K));
    map.insert_op(parse("<C-r>"), InsertRegister); // (Ctrl-R));
    map.insert_op(parse("<C-r><C-r>"), InsertRegisterContents); // (Ctrl-R Ctrl-R));
    return map;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decimal_full_match() {
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("12345g"), RemapType::Remap);
        let result = typeahead.parse_decimal();
        assert_eq!(Match::FullMatch(12345), result);
        assert_eq!(1, typeahead.len());
    }

    #[test]
    fn decimal_full_match_two() {
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("12345gg1"), RemapType::Remap);
        let result = typeahead.parse_decimal();
        assert_eq!(Match::FullMatch(12345), result);
        assert_eq!(3, typeahead.len());
    }

    #[test]
    fn decimal_partial_match() {
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("12345"), RemapType::Remap);
        let result = typeahead.parse_decimal();
        assert_eq!(Match::PartialMatch, result);
        assert_eq!(5, typeahead.len());
    }

    #[test]
    fn decimal_no_match() {
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("g12345"), RemapType::Remap);
        let result = typeahead.parse_decimal();
        assert_eq!(Match::NoMatch, result);
        assert_eq!(6, typeahead.len());
    }
}
