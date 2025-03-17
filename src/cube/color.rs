// Each variant corresponds to a specific color that can appear on the cube.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OneColor {
    White = 0,
    Green = 1,
    Red = 2,
    Orange = 3,
    Blue = 4,
    Yellow = 5,
    Invisible = 6,
}

impl OneColor {
    pub const ALL: [OneColor; 6] = [
        OneColor::White,
        OneColor::Green,
        OneColor::Red,
        OneColor::Orange,
        OneColor::Blue,
        OneColor::Yellow,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Color {
    OneColor(OneColor),
    Edge(OneColor, OneColor),
    Corner(OneColor, OneColor, OneColor),
}

pub fn get_color(color: Color) -> OneColor {
    match color {
        Color::OneColor(c) => c,
        _ => OneColor::Invisible,
    }
}

pub fn is_one_color(color: Color, one_color: OneColor) -> bool {
    match color {
        Color::OneColor(c) => c == one_color,
        _ => false,
    }
}

#[allow(dead_code)]
pub fn is_corner_color(color: Color) -> bool {
    match color {
        Color::Corner(_, _, _) => true,
        _ => false,
    }
}

pub fn convert_to_int(color: Color) -> i8 {
    match color {
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
        Color::Edge(_, _) => 0,
        Color::Corner(_, _, _) => 0
    }
}
