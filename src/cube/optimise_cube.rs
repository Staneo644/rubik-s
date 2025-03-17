use crate::cube::face::{ EFace, Face, face_to_optimize };
use crate::cube::color::Color;
use crate::cube::cube::Cube;


#[derive(Clone, Hash, Copy, Eq, PartialEq)]
pub struct Optimise_cube {
    pub faces: [u32; 6]
}

impl Optimise_cube {
    pub fn new(cube: &Cube) -> Self {
        Optimise_cube {
            faces: cube.faces.map(|face| face_to_optimize(face))
        }
    }

    pub fn print(&self) {
        for face in self.faces.iter() {
            println!("{:?}", face);
        }
    }
}