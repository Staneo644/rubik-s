use crate::cube::Cube;
use crate::cube::color::OneColor;
use crate::cube::color::Color;
use crate::cube::face::EFace;
use colored::Colorize;
use colored::ColoredString;

// Prints the top face of the cube.
impl Cube {
    pub fn print_top(&self) {
        for row in 0..3 {
            println!(
                "           {} {} {}",
                Self::print_color(self.faces[EFace::Up as usize].elements[row * 3]),
                Self::print_color(self.faces[EFace::Up as usize].elements[row * 3 + 1]),
                Self::print_color(self.faces[EFace::Up as usize].elements[row * 3 + 2])
            );
        }
        println!();
    }
}

// Prints the middle faces of the cube.
impl Cube {
    pub fn print_middle(&self) {
        let myfaces = [EFace::Left, EFace::Front, EFace::Right, EFace::Bottom];

        for row in 0..3 {
            for face in &myfaces {
                print!(
                    "  {} {} {}  ",
                    Self::print_color(self.faces[*face as usize].elements[row * 3]),
                    Self::print_color(self.faces[*face as usize].elements[row * 3 + 1]),
                    Self::print_color(self.faces[*face as usize].elements[row * 3 + 2])
                );
            }
            println!();
        }
        println!();
    }
}

// Prints the bottom face of the cube.
impl Cube {
    pub fn print_bottom(&self) {
        for row in 0..3 {
            println!(
                "           {} {} {}",
                Self::print_color(self.faces[EFace::Down as usize].elements[row * 3]),
                Self::print_color(self.faces[EFace::Down as usize].elements[row * 3 + 1]),
                Self::print_color(self.faces[EFace::Down as usize].elements[row * 3 + 2])
            );
        }
    }
}

// Prints the entire cube.
impl Cube {
    pub fn print(&self) {
        self.print_top();
        self.print_middle();
        self.print_bottom();
    }
}

// Returns a `ColoredString` representing the color for display purposes.
impl Cube {
    pub fn print_color(color: Color) -> ColoredString {
        match color {
            Color::OneColor(OneColor::White) => "W".truecolor(200, 200, 200),
            Color::OneColor(OneColor::Red) => "R".red(),
            Color::OneColor(OneColor::Blue) => "B".blue(),
            Color::OneColor(OneColor::Green) => "G".green(),
            Color::OneColor(OneColor::Orange) => "O".truecolor(255, 165, 0),
            Color::OneColor(OneColor::Yellow) => "Y".truecolor(255, 255, 0),
            Color::OneColor(OneColor::Invisible) => "E".truecolor(0, 0, 0),
            Color::Corner(c1, _, _) => {
                let color = match c1 {
                    OneColor::White => "w".truecolor(200, 200, 200),
                    OneColor::Red => "r".red(),
                    OneColor::Blue => "b".blue(),
                    OneColor::Green => "g".green(),
                    OneColor::Orange => "o".truecolor(255, 165, 0),
                    OneColor::Yellow => "y".truecolor(255, 255, 0),
                    OneColor::Invisible => "e".truecolor(0, 0, 0),
                };
                color
            }
            _ => "X".truecolor(0, 0, 0),
        }
    }
}
