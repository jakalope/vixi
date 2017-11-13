use ordered_vec_map::InsertionResult;

pub trait HasOperator<K> {
    fn insert_operator(
        &mut self,
        key: Vec<K>,
        op: OperatorOp,
    ) -> InsertionResult;
}

pub trait HasMotion<K> {
    fn insert_motion(&mut self, key: Vec<K>, op: MotionOp) -> InsertionResult;
}

pub trait HasObject<K> {
    fn insert_object(&mut self, key: Vec<K>, op: ObjectOp) -> InsertionResult;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperatorOp {
    Change, // c
    Delete, // d
    Yank, // y
    SwapCase, // ~, g~
    ToLower, // gu
    ToUpper, // gU
    ExternalPrg, // !
    EqualPrg, // =
    TextFormat, // gq
    Rot13, // g?
    ShiftRight, // >
    ShiftLeft, // <
    DefineFold, // zf
    OperatorFunc, // g@
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
    Cancel, // Drop back to normal (Esc).
    Insert, // Transitions to Insert (i).
    Repeat, // Repeats the last change (.). TODO redo-register
    Operator(OperatorOp),
    Motion(MotionOp), // Moves cursor. Transitions back to Normal.
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PendingOp {
    Cancel, // Drop back to normal (Esc).
    Operator(OperatorOp),
    Motion(MotionOp), // Cursor motions.
    Object(ObjectOp), // Text-objects.
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
    Up, // Up arrow
    Down, // Down arrow
    Left, // Left arrow
    Right, // Right arrow
}
