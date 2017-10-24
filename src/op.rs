#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NormalOp {
    Insert, // Begin insert (i).
    Delete, // Delete [motion] (d).
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InsertOp {
    Cancel, // Drop back to normal (Esc).
}
