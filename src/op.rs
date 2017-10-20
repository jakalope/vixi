#[derive(Copy, Clone)]
pub enum NormalOp {
    Insert, // Begin insert (i).
    Delete, // Delete [motion] (d).
}

#[derive(Copy, Clone)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
}
