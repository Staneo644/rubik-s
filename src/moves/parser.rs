use crate::moves::movement::{Move, MoveExtend};
use crate::moves::convert_axe::{convert_moves};
use crate::cube::{EFace, EFaceExtend};
use crate::cube::Cube;

pub struct MoveParser;


const X_ROTATION: [(EFace, EFace); 6] = [
    (EFace::Right, EFace::Right),
    (EFace::Left, EFace::Left),

    (EFace::Up, EFace::Front),
    (EFace::Front, EFace::Down),
    (EFace::Down, EFace::Bottom),
    (EFace::Bottom, EFace::Up),
];

const Y_ROTATION: [(EFace, EFace); 6] = [
    (EFace::Up, EFace::Up),
    (EFace::Down, EFace::Down),

    (EFace::Right, EFace::Bottom),
    (EFace::Bottom, EFace::Left),
    (EFace::Left, EFace::Front),
    (EFace::Front, EFace::Right),
];

const Z_ROTATION: [(EFace, EFace); 6] = [
    (EFace::Front, EFace::Front),
    (EFace::Bottom, EFace::Bottom),

    (EFace::Up, EFace::Left),
    (EFace::Left, EFace::Down),
    (EFace::Down, EFace::Right),
    (EFace::Right, EFace::Up),
];

pub fn get_value(face: &EFace, array: &[(EFace, EFace); 6]) -> EFace {
    for (f, v) in array.iter() {
        if f == face {
            return *v;
        }
    }
    *face
}

fn get_rotation (face: &mut EFace, rotation: EFaceExtend) {
    match rotation {
        EFaceExtend::X => {
            *face = get_value(face, &X_ROTATION);
        },
        EFaceExtend::Y => {
            *face = get_value(face, &Y_ROTATION);
        },
        EFaceExtend::Z => {
            *face = get_value(face, &Z_ROTATION);
        },
        _ => (),
    }
}

impl MoveParser {
    pub fn parse(sequence: &str, is_print: bool) -> Result<Vec<Move>, String> {
        let mut res = Vec::new();
        let mut rotation = Vec::new();
        let moves: Vec<MoveExtend> = sequence
            .split_whitespace()
            .map(|token| Self::parse_token(token))
            .collect::<Result<Vec<_>, _>>()?;


        if is_print {
            println!("first moves : ");
            for mv in moves.iter() {
                match mv.face {
                    EFaceExtend::EFace(face) => {
                        print!("  {:?} {}  ", face, mv.rotation);
                    },
                    _ => {
                        print!("  {:?} {}  ", mv.face, mv.rotation);
                    }
                }
            }

            println!("\n\n After :");
        }

        for mv in moves.iter() {
            match mv.face {
                EFaceExtend::X | EFaceExtend::Y | EFaceExtend::Z => {
                    rotation.push(mv);
                    if is_print {
                        print!("  {:?} {}  ", mv.face, mv.rotation);
                    }
                },
                EFaceExtend::EFace(mut face) => {
                    for rot in rotation.iter().rev() {
                        match rot.rotation {
                            1 => {
                                get_rotation(&mut face, rot.face);
                            },
                            -1 => {
                                get_rotation(&mut face, rot.face);
                                get_rotation(&mut face, rot.face);
                                get_rotation(&mut face, rot.face);
                            },
                            2 => {
                                get_rotation(&mut face, rot.face);
                                get_rotation(&mut face, rot.face);
                            },
                            _ => (),
                        }
                    }
                    res.push(Move { face: face, rotation: mv.rotation });
                    if is_print {
                        print!("  {:?} {}  ", face, mv.rotation);
                    }
                }
            }
        }
        Ok(res)
    }
}

// Implementation block for the `parse_token` function
impl MoveParser {
    fn parse_token(token: &str) -> Result<MoveExtend, String> {
        match token.chars().next() {
            Some('U') => Self::create_move(EFaceExtend::EFace(EFace::Up), token),
            Some('F') => Self::create_move(EFaceExtend::EFace(EFace::Front), token),
            Some('R') => Self::create_move(EFaceExtend::EFace(EFace::Right), token),
            Some('L') => Self::create_move(EFaceExtend::EFace(EFace::Left), token),
            Some('B') => Self::create_move(EFaceExtend::EFace(EFace::Bottom), token),
            Some('D') => Self::create_move(EFaceExtend::EFace(EFace::Down), token),
            Some('x') => Self::create_move(EFaceExtend::X, token),
            Some('y') => Self::create_move(EFaceExtend::Y, token),
            Some('z') => Self::create_move(EFaceExtend::Z, token),
            _ => Err("Invalid move notation".to_string()),
        }
    }
}

// Implementation block for the `create_move` function
impl MoveParser {
    fn create_move(face: EFaceExtend, token: &str) -> Result<MoveExtend, String> {
        let rotation = (match token.len() {
            1 => Ok(1),
            2 =>
                match token.chars().nth(1) {
                    Some('\'') => Ok(-1),
                    Some('’') => Ok(-1),
                    Some('2') => Ok(2),
                    _ => Err("Invalid move modifier".to_string()),
                }
            3 =>
                match token.chars().nth(1) {
                    Some('2') =>
                        match token.chars().nth(2) {
                            Some('\'') => Ok(2),
                            Some('’') => Ok(2),
                            _ => Err("Invalid move modifier".to_string()),
                        },
                    _ => Err("Invalid move modifier".to_string()),
                }
            _ => Err("Invalid move format".to_string()),
        })?;

        Ok(MoveExtend { face, rotation })
    }
}

#[derive(PartialEq)]
pub enum First_arg {
    Print,
    Generate,
    Nothing
}


impl MoveParser {
    pub fn parse_arguments() -> (First_arg, String) {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: {} [-step] <move_sequence>", args[0]);
            std::process::exit(1);
        }

        if args[1] == "-step" {
            if args.len() < 3 {
                eprintln!("Usage: {} [-step] <move_sequence>", args[0]);
                std::process::exit(1);
            }
            (First_arg::Print, convert_moves(&args[2]))
        } else if args[1] == "-gen" {
            (First_arg::Generate, "".to_string())
        } else
        
        {
            (First_arg::Nothing, convert_moves(&args[1]))
        }
    }
}

// Executes the given moves on the cube, optionally printing the cube state step-by-step.
impl MoveParser {
    pub fn execute_moves(cube: &mut Cube, moves: &[Move], step_by_step: bool) {
        println!("\nInitial cube state:");
        cube.print();
        for mv in moves {
            cube.apply_move(mv.face, mv.rotation);
            if step_by_step {
                println!("\nCube state after move {}:", mv);
                cube.print();
            }
        }
        println!("\nFinal cube state:");
        cube.print();
    }
}