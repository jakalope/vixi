use ordered_vec_map::InsertionResult;

pub trait HasMotion<K> {
    fn insert_motion(&mut self, key: Vec<K>, op: MotionOp) -> InsertionResult;
}

pub trait HasObject<K> {
    fn insert_object(&mut self, key: Vec<K>, op: ObjectOp) -> InsertionResult;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MotionOp {
    Left, // l
    Right, // ;
    Up, // k
    Down, // j
    Top, // gg
    Bottom, // G
    Word, // w
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjectOp {
    AWord,
    InnerWord,
    AWORD,
    InnerWORD,
    ASentence,
    InnerSentence,
    AParagraph,
    InnerParagraph,
    ASquareBlock, // A [...] block.
    InnerSquareBlock,
    AParen, // A (...) block.
    InnerParen,
    AAngle, // A <...> block.
    InnerAngle,
    ATag, // An xml tag block, e.g. <aaa>...</aaa>.
    InnerTag,
    ABrace, // A {...} block.
    InnerBrace,
    ASingleQuote, // A '...' block.
    InnerSingleQuote,
    ADoubleQuote,
    InnerDoubleQuote,
    ABackTick, // A `...` block.
    InnerBackTick,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NormalOp {
    Insert, // Insert (i). Transitions to Insert.
    Delete, // Delete (d). Transitions to Pending.
    Count(i16), // Modifies state.count.
    Motion(MotionOp), // Moves cursor. Transitions back to Normal.
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PendingOp {
    Count(i16),
    Motion(MotionOp), // Cursor motions.
    Object(ObjectOp), // Text-objects.
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
}
