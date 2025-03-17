use crate::cube::{EFace, EFaceExtend};

// The `Move` struct represents a move on the Rubik's Cube.
// It contains the face to be rotated and the rotation direction.
#[derive(Clone, PartialEq, Eq)]
pub struct Move {
    pub face: EFace,
    pub rotation: i8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct MoveExtend {
    pub face: EFaceExtend,
    pub rotation: i8,
}
