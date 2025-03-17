use crate::cube::color::Color;
use crate::cube::Cube;

// Rotates the face elements 90 degrees clockwise.
impl Cube {
    pub fn rotate(&self, elements: &[Color; 9]) -> [Color; 9] {
        let mut new_elements = *elements;
        for (i, &_element) in elements.iter().enumerate() {
            new_elements[i] = elements[
                match i {
                    0 => 2,
                    1 => 5,
                    2 => 8,
                    3 => 1,
                    5 => 7,
                    6 => 0,
                    7 => 3,
                    8 => 6,
                    _ => i,
                }
            ];
        }
        new_elements
    }
}

// Rotates the face elements 180 degrees clockwise.
impl Cube {
    pub fn rotate_double(&self, elements: &[Color; 9]) -> [Color; 9] {
        let mut new_elements = *elements;
        for (i, &_element) in elements.iter().enumerate() {
            new_elements[i] = elements[
                match i {
                    0 => 8,
                    1 => 7,
                    2 => 6,
                    3 => 5,
                    5 => 3,
                    6 => 2,
                    7 => 1,
                    8 => 0,
                    _ => i,
                }
            ];
        }
        new_elements
    }
}

// Rotates the face elements 90 degrees counterclockwise.
impl Cube {
    pub fn rotate_prime(&self, elements: &[Color; 9]) -> [Color; 9] {
        let mut new_elements = *elements;
        for (i, &_element) in elements.iter().enumerate() {
            new_elements[i] = elements[
                match i {
                    0 => 6,
                    1 => 3,
                    2 => 0,
                    3 => 7,
                    5 => 1,
                    6 => 8,
                    7 => 5,
                    8 => 2,
                    _ => i,
                }
            ];
        }
        new_elements
    }
}
