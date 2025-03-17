use crate::cube::color::Color;
use crate::cube::color::OneColor;
use crate::cube::face::EFace;
use crate::cube::Cube;

// Method to extract elements from adjacent faces.
impl Cube {
    fn extract_elements(&self, adjacent_faces: &[(EFace, [usize; 3])]) -> [Color; 12] {
        let mut temp_elements: [Color; 12] = [Color::OneColor(OneColor::White); 12];
        for (i, &(adj_face, ref indices)) in adjacent_faces.iter().enumerate() {
            for (j, &index) in indices.iter().enumerate() {
                temp_elements[i * 3 + j] = self.faces[adj_face as usize].elements[index];
            }
        }
        temp_elements
    }
}

// Calculates the new index for an element based on the rotation.
impl Cube {
    fn calculate_new_index(i: usize, j: usize, rotation: i8) -> usize {
        match rotation {
            1 => ((i + 1) % 4) * 3 + j,
            -1 => ((i + 3) % 4) * 3 + j,
            2 => ((i + 2) % 4) * 3 + j,
            _ => i * 3 + j,
        }
    }
}

// Updates the elements of the specified adjacent faces with the given colors after rotation.
impl Cube {
    fn update_elements(&mut self, adj_face: &[(EFace, [usize; 3])], colors: &[Color; 12], rot: i8) {
        for (i, &(adj_face, ref indices)) in adj_face.iter().enumerate() {
            for (j, &index) in indices.iter().enumerate() {
                let new_index = Self::calculate_new_index(i, j, rot);
                self.faces[adj_face as usize].elements[index] = colors[new_index];
            }
        }
    }
}

// Returns the adjacent faces and their indices for the bottom face.
impl Cube {
    fn get_adjacent_faces_down(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Front, [6, 7, 8]),
            (EFace::Left, [6, 7, 8]),
            (EFace::Bottom, [6, 7, 8]),
            (EFace::Right, [6, 7, 8]),
        ]
    }
}

// Returns the adjacent faces and their indices for the top face.
impl Cube {
    fn get_adjacent_faces_up(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Front, [0, 1, 2]),
            (EFace::Right, [0, 1, 2]),
            (EFace::Bottom, [0, 1, 2]),
            (EFace::Left, [0, 1, 2]),
        ]
    }
}

// Returns the adjacent faces and their indices for the left face.
impl Cube {
    fn get_adjacent_faces_front(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Left, [2, 5, 8]),
            (EFace::Down, [0, 1, 2]),
            (EFace::Right, [6, 3, 0]),
            (EFace::Up, [8, 7, 6]),
        ]
    }
}

// Returns the adjacent faces and their indices for the right face.
impl Cube {
    fn get_adjacent_faces_right(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Front, [2, 5, 8]),
            (EFace::Down, [2, 5, 8]),
            (EFace::Bottom, [6, 3, 0]),
            (EFace::Up, [2, 5, 8]),
        ]
    }
}

// Returns the adjacent faces and their indices for the left face.
impl Cube {
    fn get_adjacent_faces_left(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Bottom, [8, 5, 2]),
            (EFace::Down, [0, 3, 6]),
            (EFace::Front, [0, 3, 6]),
            (EFace::Up, [0, 3, 6]),
        ]
    }
}

// Returns the adjacent faces and their indices for the back face.
impl Cube {
    fn get_adjacent_faces_bottom(&self) -> [(EFace, [usize; 3]); 4] {
        [
            (EFace::Right, [2, 5, 8]),
            (EFace::Down, [8, 7, 6]),
            (EFace::Left, [6, 3, 0]),
            (EFace::Up, [0, 1, 2]),
        ]
    }
}

// Method to update elements of adjacent faces after a rotation.
impl Cube {
    pub fn update_adjacent_faces(&mut self, face: EFace, rotation: i8) {
        let adjacent_faces = match face {
            EFace::Down => self.get_adjacent_faces_down(),
            EFace::Up => self.get_adjacent_faces_up(),
            EFace::Front => self.get_adjacent_faces_front(),
            EFace::Right => self.get_adjacent_faces_right(),
            EFace::Left => self.get_adjacent_faces_left(),
            EFace::Bottom => self.get_adjacent_faces_bottom(),
        };

        let temp_elements = self.extract_elements(&adjacent_faces);
        self.update_elements(&adjacent_faces, &temp_elements, rotation);
    }
}
