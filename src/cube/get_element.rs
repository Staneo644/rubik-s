use crate::cube::face::{ EFace };
use crate::cube::Cube;
use crate::cube::color::{ OneColor, Color, get_color, is_one_color };

const EDGES: [((EFace, usize), (EFace, usize)); 12] = [
    ((EFace::Up, 1), (EFace::Bottom, 1)),
    ((EFace::Up, 3), (EFace::Left, 1)),
    ((EFace::Up, 5), (EFace::Right, 1)),
    ((EFace::Up, 7), (EFace::Front, 1)),
    ((EFace::Front, 7), (EFace::Down, 1)),
    ((EFace::Left, 7), (EFace::Down, 3)),
    ((EFace::Right, 7), (EFace::Down, 5)),
    ((EFace::Bottom, 7), (EFace::Down, 7)),
    ((EFace::Front, 3), (EFace::Left, 5)),
    ((EFace::Front, 5), (EFace::Right, 3)),
    ((EFace::Bottom, 5), (EFace::Left, 3)),
    ((EFace::Bottom, 3), (EFace::Right, 5)),
];

pub const CORNER: [((EFace, usize), (EFace, usize), (EFace, usize)); 8] = [
    ((EFace::Up, 0), (EFace::Bottom, 2), (EFace::Left, 0)),
    ((EFace::Up, 2), (EFace::Right, 2), (EFace::Bottom, 0)),
    ((EFace::Up, 6), (EFace::Front, 0), (EFace::Left, 2)),
    ((EFace::Up, 8), (EFace::Front, 2), (EFace::Right, 0)),
    ((EFace::Down, 0), (EFace::Front, 6), (EFace::Left, 8)),
    ((EFace::Down, 2), (EFace::Front, 8), (EFace::Right, 6)),
    ((EFace::Down, 6), (EFace::Bottom, 8), (EFace::Left, 6)),
    ((EFace::Down, 8), (EFace::Bottom, 6), (EFace::Right, 8)),
];

pub const EDGES_LIST: [(EFace, usize); 24] = [
    (EFace::Up, 1),
    (EFace::Up, 3),
    (EFace::Up, 5),
    (EFace::Up, 7),
    (EFace::Front, 1),
    (EFace::Front, 3),
    (EFace::Front, 5),
    (EFace::Front, 7),
    (EFace::Right, 1),
    (EFace::Right, 3),
    (EFace::Right, 5),
    (EFace::Right, 7),
    (EFace::Left, 1),
    (EFace::Left, 3),
    (EFace::Left, 5),
    (EFace::Left, 7),
    (EFace::Bottom, 1),
    (EFace::Bottom, 3),
    (EFace::Bottom, 5),
    (EFace::Bottom, 7),
    (EFace::Down, 1),
    (EFace::Down, 3),
    (EFace::Down, 5),
    (EFace::Down, 7),
];

fn not_colors(
    cube: &Cube,
    ((face1, pos1), (face2, pos2), (face3, pos3)): ((EFace, usize), (EFace, usize), (EFace, usize)),
    reject_colors: &[OneColor]
) -> bool {
    !reject_colors.contains(&get_color(*cube.get_element(face1, pos1))) &&
        !reject_colors.contains(&get_color(*cube.get_element(face2, pos2))) &&
        !reject_colors.contains(&get_color(*cube.get_element(face3, pos3)))
}

fn true_fn(_: &Cube, _: ((EFace, usize), (EFace, usize), (EFace, usize)), _: &[OneColor]) -> bool {
    true
}

fn not_or_colors(
    cube: &Cube,
    ((face1, pos1), (face2, pos2), (face3, pos3)): ((EFace, usize), (EFace, usize), (EFace, usize)),
    reject_colors: &[OneColor]
) -> bool {
    !(
        reject_colors.contains(&get_color(*cube.get_element(face1, pos1))) &&
        reject_colors.contains(&get_color(*cube.get_element(face2, pos2))) &&
        reject_colors.contains(&get_color(*cube.get_element(face3, pos3)))
    )
}

#[allow(dead_code)]
impl Cube {
    pub fn get_edges_color(&self, color: OneColor) -> Vec<(EFace, usize)> {
        let mut edges_color = Vec::new();

        for &((face1, pos1), (face2, pos2)) in &EDGES {
            if is_one_color(self.faces[face1 as usize].elements[pos1], color) {
                edges_color.push((face1, pos1));
                edges_color.push((face2, pos2));
            } else if is_one_color(self.faces[face2 as usize].elements[pos2], color) {
                edges_color.push((face2, pos2));
                edges_color.push((face1, pos1));
            }
        }
        edges_color
    }

    pub fn get_edges_color_not_one(
        &self,
        color: OneColor,
        reject_color: OneColor
    ) -> Vec<(EFace, usize)> {
        let mut edges_color = Vec::new();

        for &((face1, pos1), (face2, pos2)) in &EDGES {
            if
                is_one_color(self.faces[face1 as usize].elements[pos1], color) &&
                !is_one_color(self.faces[face2 as usize].elements[pos2], reject_color)
            {
                edges_color.push((face1, pos1));
                edges_color.push((face2, pos2));
            } else if
                is_one_color(self.faces[face2 as usize].elements[pos2], color) &&
                !is_one_color(self.faces[face1 as usize].elements[pos1], reject_color)
            {
                edges_color.push((face2, pos2));
                edges_color.push((face1, pos1));
            }
        }
        edges_color
    }

    pub fn get_edges_not_one_color(&self, reject_color: OneColor) -> Vec<(EFace, usize)> {
        let mut edges_color = Vec::new();

        for &((face1, pos1), (face2, pos2)) in &EDGES {
            if
                !is_one_color(self.faces[face1 as usize].elements[pos1], reject_color) &&
                !is_one_color(self.faces[face2 as usize].elements[pos2], reject_color)
            {
                edges_color.push((face1, pos1));
                edges_color.push((face2, pos2));
            }
        }
        edges_color
    }

    pub fn get_edges_not_colors_array(&self, reject_colors: &[OneColor]) -> Vec<(EFace, usize)> {
        let mut edges_color = Vec::new();

        for &((face1, pos1), (face2, pos2)) in &EDGES {
            if
                !reject_colors.contains(&get_color(self.faces[face1 as usize].elements[pos1])) ||
                !reject_colors.contains(&get_color(self.faces[face2 as usize].elements[pos2]))
            {
                edges_color.push((face1, pos1));
                edges_color.push((face2, pos2));
            }
        }
        edges_color
    }

    pub fn get_corners_color<F>(
        &self,
        cube: Cube,
        color: Color,
        reject_colors: &[OneColor],
        not_colors_fn: F
    ) -> Vec<(EFace, usize)>
        where F: Fn(&Cube, ((EFace, usize), (EFace, usize), (EFace, usize)), &[OneColor]) -> bool
    {
        let mut corners_color = Vec::new();

        for &((face1, pos1), (face2, pos2), (face3, pos3)) in &CORNER {
            if not_colors_fn(&cube, ((face1, pos1), (face2, pos2), (face3, pos3)), reject_colors) {
                if self.faces[face1 as usize].elements[pos1] == color {
                    corners_color.push((face1, pos1));
                    corners_color.push((face2, pos2));
                    corners_color.push((face3, pos3));
                } else if self.faces[face2 as usize].elements[pos2] == color {
                    corners_color.push((face2, pos2));
                    corners_color.push((face3, pos3));
                    corners_color.push((face1, pos1));
                } else if self.faces[face3 as usize].elements[pos3] == color {
                    corners_color.push((face3, pos3));
                    corners_color.push((face1, pos1));
                    corners_color.push((face2, pos2));
                }
            }
        }
        corners_color
    }

    pub fn is_zbll(&self, face: EFace) -> bool {
        let cube = Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(get_color(self.faces[face as usize].elements[4])),
                    &[],
                    true_fn
                ),
                EDGES_LIST.to_vec(),
            ].concat()
        );

        let default_cube = Cube::new_with_elements(
            Cube::new(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(get_color(self.faces[face as usize].elements[4])),
                    &[],
                    true_fn
                ),
                EDGES_LIST.to_vec(),
            ].concat()
        );
        cube == default_cube
    }

    pub fn get_zbll(&self) -> EFace {
        let mut face = EFace::Up;
        for _i in 0..6 {
            if self.is_zbll(face) {
                return face;
            }
            face = face.next();
        }
        EFace::Up
    }

    ////////////////////////////////////////////

    pub fn get_white_cross_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_edges_color(OneColor::White),
                self.get_edges_color_not_one(OneColor::Orange, OneColor::White),
            ].concat()
        )
    }

    pub fn get_white_cross_default() -> Cube {
        let cube = Cube::new();
        cube.get_white_cross_cube()
    }

    ////////////////////////////////////////////

    pub fn get_first_layer_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[self.get_edges_not_one_color(OneColor::Yellow)].concat()
        )
    }

    pub fn get_first_layer_default() -> Cube {
        let cube = Cube::new();
        cube.get_first_layer_cube()
    }

    ////////////////////////////////////////////

    pub fn get_one_white_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[OneColor::Blue, OneColor::Red],
                    not_colors
                ),
                self.get_edges_not_one_color(OneColor::Yellow),
            ].concat()
        )
    }

    pub fn get_one_white_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_one_white_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_two_white_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[OneColor::Blue],
                    not_colors
                ),
                self.get_edges_not_one_color(OneColor::Yellow),
            ].concat()
        )
    }

    pub fn get_two_white_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_two_white_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_three_white_corner_cube(&self, array: &[OneColor]) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    array,
                    not_or_colors
                ),
                self.get_edges_not_one_color(OneColor::Yellow),
            ].concat()
        )
    }

    pub fn get_three_white_corner_cube_static(cube: Cube, array: &[OneColor]) -> Cube {
        cube.get_three_white_corner_cube(array)
    }

    pub fn get_three_white_corner_default(array: &[OneColor]) -> Cube {
        let cube = Cube::new();
        cube.get_three_white_corner_cube(array)
    }

    ////////////////////////////////////////////

    pub fn get_last_white_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                self.get_edges_not_one_color(OneColor::Yellow),
            ].concat()
        )
    }

    pub fn get_last_white_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_last_white_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_two_yellow_edges_cube(&self, color_array: &[OneColor]) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                self.get_edges_not_colors_array(color_array),
            ].concat()
        )
    }

    pub fn get_two_yellow_edges_cube_static(cube: Cube, color_array: &[OneColor]) -> Cube {
        cube.get_two_yellow_edges_cube(color_array)
    }

    pub fn get_two_yellow_edges_default(color_array: &[OneColor]) -> Cube {
        let cube = Cube::new();
        cube.get_two_yellow_edges_cube(color_array)
    }

    ////////////////////////////////////////////

    pub fn get_edges_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                EDGES_LIST.to_vec(),
            ].concat()
        )
    }

    pub fn get_edges_default() -> Cube {
        let cube = Cube::new();
        cube.get_edges_cube()
    }

    ////////////////////////////////////////////

    pub fn get_first_green_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::Green),
                    &[OneColor::White, OneColor::Red],
                    not_colors
                ),
                EDGES_LIST.to_vec(),
            ].concat()
        )
    }

    pub fn get_first_green_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_first_green_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_green_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                EDGES_LIST.to_vec(),
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::Green),
                    &[OneColor::White],
                    not_colors
                ),
            ].concat()
        )
    }

    pub fn get_green_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_green_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_first_yellow_corner_cube(&self) -> Cube {
        Cube::new_with_elements(
            self.clone(),
            &[
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    &[],
                    true_fn
                ),
                EDGES_LIST.to_vec(),
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::Yellow),
                    &[OneColor::Red, OneColor::Blue, OneColor::Yellow],
                    not_or_colors
                ),
            ].concat()
        )
    }

    pub fn get_first_yellow_corner_default() -> Cube {
        let cube = Cube::new();
        cube.get_first_yellow_corner_cube()
    }

    ////////////////////////////////////////////

    pub fn get_good_place_corner_cube(&self, color_array: &[OneColor]) -> Cube {
        Cube::new_with_elements_corner_tuple(
            self.clone(),
            &[
                EDGES_LIST.to_vec(),
                self.get_corners_color(
                    self.clone(),
                    Color::OneColor(OneColor::White),
                    color_array,
                    not_or_colors
                ),
            ].concat()
        )
    }

    pub fn get_good_place_corner_default(color_array: &[OneColor]) -> Cube {
        let cube = Cube::new();
        cube.get_good_place_corner_cube(color_array)
    }

    pub fn get_good_place_corner_cube_static(cube: Cube, color_array: &[OneColor]) -> Cube {
        cube.get_good_place_corner_cube(color_array)
    }
}
