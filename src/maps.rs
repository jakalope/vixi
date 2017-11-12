use disambiguation_map::Match;
use key::parse::parse;
use key::{MultiKey, Key};
use mode_map::ModeMap;
use op::{HasOperator, HasMotion, HasObject, PendingOp, ObjectOp, MotionOp,
         InsertOp, NormalOp, OperatorOp};
use ordered_vec_map::InsertionResult;
use state::NumericMap;
use typeahead::{RemapType, Typeahead};

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

struct DecimalMap {}

impl NumericMap<MultiKey> for DecimalMap {
    /// Parses the front of the typeahead buffer for a non-negative decimal
    /// integer string.
    ///
    /// Returns
    /// * `Match::FullMatch(N)` if a non-empty numeric string of
    ///   value `N` was found, followed by a non-numeric character. The
    ///   non-numeric character implies the numeric string is complete.
    /// * `Match::PartialMatch` if the typeahead buffer contains only
    ///   decimal digits. This implies the numeric string might not be
    ///   complete.
    /// * `Match::NoMatch` if the first character in the typeahead buffer is
    ///   non-numeric.
    fn process(&self, typeahead: &mut Typeahead<MultiKey>) -> Match<i32> {
        let mut s = String::with_capacity(typeahead.len());
        let mut found_non_digit = false;
        for key in typeahead.value_iter() {
            match key {
                MultiKey::A(Key::Char(c)) => {
                    if c.is_digit(10) {
                        s.push(c);
                    } else {
                        found_non_digit = true;
                        break;
                    }
                }
                _ => {
                    found_non_digit = true;
                    break;
                }
            }
        }
        if found_non_digit {
            if s.is_empty() {
                return Match::NoMatch;
            } else {
                for i in 0..s.len() {
                    typeahead.pop_front();
                }
                return Match::FullMatch(s.parse::<i32>().unwrap());
            }
        }
        return Match::PartialMatch;
    }
}

pub fn numeric_map() -> Box<NumericMap<MultiKey>> {
    Box::new(DecimalMap {})
}

pub fn normal_mode_map() -> ModeMap<MultiKey, NormalOp> {
    use op::NormalOp::*;
    let mut map = ModeMap::new();
    map.insert_op(parse("<Esc>"), Cancel);
    map.insert_op(parse("i"), Insert);
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
    return map;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_numeric_full_match() {
        let map = DecimalMap {};
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("12345g"), RemapType::Remap);
        let result = map.process(&mut typeahead);
        assert_eq!(Match::FullMatch(12345), result);
        assert_eq!(1, typeahead.len());
    }

    #[test]
    fn parse_numeric_partial_match() {
        let map = DecimalMap {};
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("12345"), RemapType::Remap);
        let result = map.process(&mut typeahead);
        assert_eq!(Match::PartialMatch, result);
        assert_eq!(5, typeahead.len());
    }

    #[test]
    fn parse_numeric_no_match() {
        let map = DecimalMap {};
        let mut typeahead = Typeahead::new();
        typeahead.put_front(&parse("g12345"), RemapType::Remap);
        let result = map.process(&mut typeahead);
        assert_eq!(Match::NoMatch, result);
        assert_eq!(6, typeahead.len());
    }
}