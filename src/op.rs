#[derive(Copy, Clone)]
pub enum NormalOp {
    Insert, // Enter Insert mode (i).
}

#[derive(Copy, Clone)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
}
