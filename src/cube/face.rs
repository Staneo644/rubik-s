use super::color::Color;
use crate::cube::color::OneColor;
// The `EFace` enum represents the six faces of a Rubik's Cube.
// Each variant corresponds to a specific face of the cube.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EFace {
    Up = 0,
    Front = 1,
    Right = 2,
    Left = 3,
    Bottom = 4,
    Down = 5,
}

impl EFace {
    pub fn next(self) -> Self {
        match self {
            EFace::Up => EFace::Front,
            EFace::Front => EFace::Right,
            EFace::Right => EFace::Left,
            EFace::Left => EFace::Bottom,
            EFace::Bottom => EFace::Down,
            EFace::Down => EFace::Up,
        }
    }
}

pub fn face_to_optimize(face: Face) -> u32 {
    let mut result = 0;
    for element in face.elements.iter() {
        let element_num = match element {
            Color::OneColor(c) => {
                match c {
                    OneColor::White => 0,
                    OneColor::Green => 1,
                    OneColor::Red => 2,
                    OneColor::Orange => 3,
                    OneColor::Blue => 4,
                    OneColor::Yellow => 5,
                    OneColor::Invisible => 6,
                }
            }
            Color::Edge(_, _) => 6,
            Color::Corner(_, _, _) => 6
        };
        result *= 7;
        result += element_num;
    }
    result
}

pub fn optimize_to_face(mut num: u32) -> Face {
    let mut result = [Color::OneColor(OneColor::Invisible); 9];
    for i in 0..9 {
        let element_num = num % 7;
        num /= 7;
        let element = match element_num {
            0 => Color::OneColor(OneColor::White),
            1 => Color::OneColor(OneColor::Green),
            2 => Color::OneColor(OneColor::Red),
            3 => Color::OneColor(OneColor::Orange),
            4 => Color::OneColor(OneColor::Blue),
            5 => Color::OneColor(OneColor::Yellow),
            6 => Color::OneColor(OneColor::Invisible),
            _ => panic!("Invalid color number"),
        };
        result[8 - i] = element;
    }
    Face { elements: result }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EFaceExtend {
    EFace(EFace),
    X = 6,
    Y = 7,
    Z = 8,
}
// The `Face` struct represents a single face of a Rubik's Cube.
// It contains an array of `Color` elements representing the colors of the face.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Face {
    pub(super) elements: [Color; 9],
}
