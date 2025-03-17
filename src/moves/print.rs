use crate::moves::movement::Move;
use crate::cube::EFace;
use std::fmt;

// Implementation of the `fmt::Display` trait for the `Move` struct.
// This allows a `Move` to be formatted as a string for display purposes.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}", self.face, self.rotation)
    }
}

// Function to convert a move to its official Rubik's Cube notation.
fn move_to_notation(mv: &Move) -> String {
    let face_notation = match mv.face {
        EFace::Front => "F",
        EFace::Right => "R",
        EFace::Up => "U",
        EFace::Bottom => "D",
        EFace::Left => "L",
        EFace::Down => "B",
    };

    let rotation_notation = match mv.rotation {
        1 => "".to_string(),
        2 => "2".to_string(),
        -1 => "'".to_string(),
        _ => format!("{}", mv.rotation),
    };

    format!("{}{}", face_notation, rotation_notation)
}

// Function to transform a list of moves into the official Rubik's Cube notation.
pub fn transform_moves_to_notation(moves: &[Move]) -> String {
    moves.iter().map(|mv| move_to_notation(mv)).collect::<Vec<_>>().join(" ")
}