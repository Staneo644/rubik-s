use crate::cube::color::OneColor;
use crate::cube::color::{ Color, get_color };
use crate::cube::face::{ EFace, Face, optimize_to_face };
use crate::cube::get_element::CORNER;
use crate::cube::optimise_cube::Optimise_cube;


// The `Cube` struct represents a Rubik's Cube with six faces.
// Each face is represented by a `Face` struct containing an array of `OneColor` elements.
const CORNER_TOP : [[OneColor; 3]; 4] = [
    [OneColor::White, OneColor::Blue, OneColor::Orange],
    [OneColor::White, OneColor::Red, OneColor::Blue],
    [OneColor::White, OneColor::Green, OneColor::Orange],
    [OneColor::White, OneColor::Green, OneColor::Red]
];

pub const OTHER_CORNER: [[(EFace, usize); 3]; 4] = [
    [(EFace::Up, 0), (EFace::Bottom, 2), (EFace::Left, 0)],
    [(EFace::Up, 2), (EFace::Right, 2), (EFace::Bottom, 0)],
    [(EFace::Up, 6), (EFace::Front, 0), (EFace::Left, 2)],
    [(EFace::Up, 8), (EFace::Front, 2), (EFace::Right, 0)]
];

const PERMUTATIONS : [[i8; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0]
];

const num_states: i8 = 3;
#[derive(Clone, Hash, Copy)]
pub struct Cube {
    pub faces: [Face; 6]
}

// Default implementation for the `Cube` struct.
// Provides a method to create a new `Cube` with default face colors.
impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_color_range(colors: Vec<OneColor>, mut item: i16) -> OneColor {
    for color in OneColor::ALL.iter() {
        if colors.contains(&color) {
            item -= 1;
            if item == 0 {
                return color.clone();
            }
        }
    }
    OneColor::Invisible
}

// Creates a new `Cube` with default face colors.
#[allow(dead_code)]
impl Cube {
    pub fn new() -> Self {
        Cube {
            faces: [
                Face { elements: [Color::OneColor(OneColor::White); 9] },
                Face { elements: [Color::OneColor(OneColor::Green); 9] },
                Face { elements: [Color::OneColor(OneColor::Red); 9] },
                Face { elements: [Color::OneColor(OneColor::Orange); 9] },
                Face { elements: [Color::OneColor(OneColor::Blue); 9] },
                Face { elements: [Color::OneColor(OneColor::Yellow); 9] },
            ]
        }
    }

    pub fn new_optimized(optimise_cube: &Optimise_cube) -> Self {
        Cube {
            faces : optimise_cube.faces.map(|face| optimize_to_face(face))
        }
    }

    pub fn new_with_elements(cube: Cube, positions: &[(EFace, usize)]) -> Self {
        let mut faces = [Face { elements: [Color::OneColor(OneColor::Invisible); 9] }; 6];
        for (face, position) in positions {
            faces[*face as usize].elements[*position] = cube.faces[*face as usize].elements[*position];
        }
        Cube {
            faces
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for face in &self.faces {
            for element in &face.elements {
                result.push_str(&Cube::print_color(*element));
            }
            result.push('-');
        }
        result.pop();
        result
    }

    pub fn is_face_complete(&self, face: EFace) -> bool {
        let elements = &self.faces[face as usize].elements;
        let color = elements[0];
        for element in elements {
            if *element != color {
                return false;
            }
        }
        true
    }
    

    fn generate_cube(corner_permutation: [[OneColor; 3]; 4], corner_orientation: [i8; 4]) -> Cube {
        let mut cube = Cube::get_edges_default();
        for i in 0..4 {
            cube.set_element(OTHER_CORNER[i][0].0, OTHER_CORNER[i][0].1, Color::OneColor(corner_permutation[i][((0 + corner_orientation[i]) % 3) as usize]));
            cube.set_element(OTHER_CORNER[i][1].0, OTHER_CORNER[i][1].1, Color::OneColor(corner_permutation[i][((1 + corner_orientation[i]) % 3) as usize]));
            cube.set_element(OTHER_CORNER[i][2].0, OTHER_CORNER[i][2].1, Color::OneColor(corner_permutation[i][((2 + corner_orientation[i]) % 3) as usize]));
        }
        cube
    }

    pub fn generate_all_randoms_cube() -> Vec<Cube> {
        let mut result = Vec::new();

        for permutation in PERMUTATIONS {        
            let mut corner_permutation : [[OneColor; 3]; 4] = [[OneColor::Invisible; 3]; 4];
            for i in 0..4 {
                corner_permutation[i] = CORNER_TOP[permutation[i] as usize];
            }

            for a in 0..num_states {
                for b in 0..num_states {
                    for c in 0..num_states {
                        for d in 0..num_states {
                            result.push(Cube::generate_cube(corner_permutation, [a, b, c, d]));
                        }
                    }
                }
            }
        }

        
        result
    }

    /*pub fn refresh_bottom_cube(&self) -> Cube {
        let mut new_cube = self.clone();
        for i in 0..9 {
            new_cube.faces[EFace::Down as usize].elements[i] = Color::OneColor(OneColor::Invisible);
        }
        for i in 4..9 {
            new_cube.faces[EFace::Front as usize].elements[i] = Color::OneColor(OneColor::Invisible);
            new_cube.faces[EFace::Right as usize].elements[i] = Color::OneColor(OneColor::Invisible);
            new_cube.faces[EFace::Bottom as usize].elements[i] = Color::OneColor(OneColor::Invisible);
            new_cube.faces[EFace::Left as usize].elements[i] = Color::OneColor(OneColor::Invisible);
        }
        new_cube
    }*/

    pub fn new_with_elements_corner_tuple(cube: Cube, positions: &[(EFace, usize)]) -> Self {
        //println!(">>>>>>>>>>>>>>>>>>>  {} <<<<<<<<<<<<<<<<<", positions.len());
        let mut faces = [Face { elements: [Color::OneColor(OneColor::Invisible); 9] }; 6];
        for (face, position) in positions {
            faces[*face as usize].elements[*position] = cube.faces[*face as usize].elements[*position];
        }
        for &((face1, pos1), (face2, pos2), (face3, pos3)) in &CORNER {
            let corner = Color::Corner(
                get_color_range(
                    vec!(
                        get_color(faces[face1 as usize].elements[pos1]), 
                        get_color(faces[face2 as usize].elements[pos2]), 
                        get_color(faces[face3 as usize].elements[pos3])
                    ), 
                    1
                ),
                OneColor::Invisible,
                OneColor::Invisible
            );
            faces[face1 as usize].elements[pos1] = corner;
            faces[face2 as usize].elements[pos2] = corner;
            faces[face3 as usize].elements[pos3] = corner;
        }
        Cube {
            faces
        }
    }

    pub fn set_corner(&mut self, face: EFace, position: usize, color: Color) {
        for &((face1, pos1), (face2, pos2), (face3, pos3)) in &CORNER {
            if face == face1 && position == pos1 ||
               face == face2 && position == pos2 ||
               face == face3 && position == pos3 {
                self.faces[face1 as usize].elements[pos1] = color;
                self.faces[face2 as usize].elements[pos2] = color;
                self.faces[face3 as usize].elements[pos3] = color;
            }
        }
    }

    pub fn get_element(&self, face: EFace, position: usize) -> &Color {
        &self.faces[face as usize].elements[position]
    }

    pub fn set_element(&mut self, face: EFace, position: usize, color: Color) {
        self.faces[face as usize].elements[position] = color;
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Eq for Cube {
}

// Applies a move to the `Cube` by rotating the specified face and updating adjacent faces.
impl Cube {
    pub fn apply_move(&mut self, face: EFace, rotation: i8) {
        let elements = self.faces[face as usize].elements;
        let new_elements = match rotation {
            -1 => self.rotate(&elements),
            1 => self.rotate_prime(&elements),
            2 => self.rotate_double(&elements),
            _ => elements,
        };
        self.faces[face as usize].elements = new_elements;
        self.update_adjacent_faces(face, rotation);
    }
}
