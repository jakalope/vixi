#[derive(Copy, Clone)]
pub enum NormalOp {
    Insert, // Enter Insert mode (i).
    Append, // Move cursor right and enter Insert (a).
    AppendAtEnd, // Move cursor to the far right and enter Insert (A).
}

#[derive(Copy, Clone)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
}
