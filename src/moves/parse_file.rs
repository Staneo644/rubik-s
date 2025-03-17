use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use std::collections::HashMap;
use crate::cube::Cube;
use crate::moves::movement::{ Move };
use crate::moves::convert_axe::{ convert_moves };
use crate::cube::{ EFace };
use crate::moves::parser::{ MoveParser };
use std::io::Write;

const ORDER: [EFace; 4] = [EFace::Front, EFace::Right, EFace::Bottom, EFace::Left];
const FOLDER: &str = "./zbll-list/cubing-app/";

fn parse_moves(moves: &Vec<Move>) -> [Vec<Move>; 4] {
    let mut res = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for i in 0..4 {
        for mv in moves.iter() {
            let mut find = false;
            for k in 0..4 {
                if ORDER[k] == mv.face {
                    res[i].push(Move { face: ORDER[(k + i) % 4], rotation: mv.rotation });
                    find = true;
                }
            }
            if !find {
                res[i].push(mv.clone());
            }
        }
    }
    res
}

fn face_to_string(face: &EFace) -> &str {
    match face {
        EFace::Front => "F",
        EFace::Right => "R",
        EFace::Bottom => "B",
        EFace::Left => "L",
        EFace::Up => "U",
        EFace::Down => "D",
    }
}

pub fn vec_move_to_string(moves: &Vec<Move>) -> String {
    let mut res = String::new();
    for mv in moves.iter() {
        res.push_str(&format!("{}{} ", face_to_string(&mv.face), mv.rotation));
    }
    res
}

pub fn parse_line(line: &str, reverse_line: &str, index_line: usize, reverse: bool) -> [(Cube, Vec<Move>); 4] {
    let mut res = [
        (Cube::new(), Vec::new()),
        (Cube::new(), Vec::new()),
        (Cube::new(), Vec::new()),
        (Cube::new(), Vec::new()),
    ];
    match MoveParser::parse(&convert_moves(line), false) 
    {
        Ok(moves) => 
        {
            let mut arrays = parse_moves(&moves);
            let mut arrays_reverse = parse_moves(
                &MoveParser::parse(&convert_moves(reverse_line), false).unwrap()
            );
            for i in 0..4 
            {
                let mut cube = Cube::new();
                for mv in &arrays_reverse[i].clone() {
                    cube.apply_move(mv.face, mv.rotation);
                }
                if (!reverse && !cube.is_zbll(EFace::Up)) || (reverse && !cube.is_zbll(EFace::Down))
                {
                    match cube.get_zbll() 
                    {
                        EFace::Front => 
                        {
                            return parse_line(
                                &format!("x' {}", line),
                                &format!("x' {}", reverse_line),
                                index_line,
                                reverse
                            );
                        }
                        EFace::Bottom => {
                            return parse_line(
                            &format!("x {}", line),
                            &format!("x {}", reverse_line),
                            index_line,
                            reverse
                        );
                        }
                        EFace::Right => {
                            return parse_line(
                                &format!("z {}", line),
                                &format!("z {}", reverse_line),
                                index_line,
                                reverse
                            );
                        }
                        EFace::Left => {
                            return parse_line(
                                &format!("z' {}", line),
                                &format!("z' {}", reverse_line),
                                index_line,
                                reverse
                            );
                        }
                        EFace::Down => {
                            return parse_line(
                                &format!("x2 {}", line),
                                &format!("x2 {}", reverse_line),
                                index_line,
                                reverse
                            );
                        }
                        _ => {
                            let mut move_to_do = EFace::Down;
                            let mut zbll_test = EFace::Up;
                            if reverse {
                                move_to_do = EFace::Up;
                                zbll_test = EFace::Down;
                            }
                            cube.apply_move(move_to_do, 1);
                            if cube.is_zbll(zbll_test) {
                                arrays[i].insert(0, Move { face: move_to_do, rotation: -1 });
                                arrays_reverse[i].push(Move { face: zbll_test, rotation: 1 });
                            } else {
                                cube.apply_move(move_to_do, 1);
                                if cube.is_zbll(zbll_test) {
                                    arrays[i].insert(0, Move { face: move_to_do, rotation: 2 });
                                    arrays_reverse[i].push(Move { face: move_to_do, rotation: 2 });
                                } else {
                                    cube.apply_move(move_to_do, 1);
                                    if cube.is_zbll(zbll_test) {
                                        arrays[i].insert(0, Move {
                                            face: move_to_do,
                                            rotation: 1,
                                        });
                                        arrays_reverse[i].push(Move {
                                            face: move_to_do,
                                            rotation: -1,
                                        });
                                    } else {
                                        arrays[i].truncate(arrays[i].len().saturating_sub(3));
                                        arrays_reverse[i].truncate(
                                            arrays_reverse[i].len().saturating_sub(3)
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                res[i] = (cube, arrays[i].clone());
            }
        }
        Err(err) => {
            eprintln!("Error parsing moves: {} at line: {}", err, index_line);
        }
    }
    res
}

pub fn open_file() -> (HashMap<Cube, Vec<Move>>, HashMap<Cube, Vec<Move>>) {
    let zbll_path = FOLDER.to_owned() + "zbll.txt";
    let path = Path::new(&zbll_path);
    let reverse_zbll_path = FOLDER.to_owned() + "zbll-reverse.txt";
    let reverse_path = Path::new(&reverse_zbll_path);
    let mut new_file = File::create(&(FOLDER.to_owned() + "zbll-parse.txt")).expect(
        "Failed to create file"
    );
    let mut new_file_yellow = File::create(&(FOLDER.to_owned() + "zbll-parse-yellow.txt")).expect(
        "Failed to create file"
    );
    let file = File::open(&path).expect("Failed to open file");
    let mut res = HashMap::new();
    let mut res_yellow = HashMap::new();
    let reverse_file = File::open(&reverse_path).expect("Failed to open reverse file");

    let reverse_reader = io::BufReader::new(reverse_file).lines();
    let reader = io::BufReader::new(file).lines();
    let mut index_line = 0;
    for (line, reverse_line) in reader.zip(reverse_reader) {
        index_line += 1;
        let new_line_yellow = format!("{}", line.unwrap());
        let new_line = format!("x2 {}", new_line_yellow);

        let reverse_line_unwrap_yellow = format!("{}", reverse_line.unwrap());
        let reverse_line_unwrap = format!("x2 {}", reverse_line_unwrap_yellow);
        let cubes_and_moves_yellow = parse_line(&new_line_yellow, &reverse_line_unwrap_yellow, index_line, true);
        let cubes_and_moves = parse_line(&new_line, &reverse_line_unwrap, index_line, false);

        for i in 0..4 {
            let (cube, moves) = &cubes_and_moves[i];
            let cube_clone = cube.clone();
            let line = format!("{}: {}\n", cube.to_string(), vec_move_to_string(&moves));
            new_file.write_all(line.as_bytes()).expect("Failed to write to file");
            if !res.contains_key(&cube_clone) {
                res.insert(cube_clone, moves.to_vec());
            }

            let (cube_yellow, moves_yellow) = &cubes_and_moves_yellow[i];
            let cube_clone_yellow = cube_yellow.clone();
            let line_yellow = format!("{}: {}\n", cube_yellow.to_string(), vec_move_to_string(&moves_yellow));
            new_file_yellow.write_all(line_yellow.as_bytes()).expect("Failed to write to file");
            if !res_yellow.contains_key(&cube_clone_yellow) {
                res_yellow.insert(cube_clone_yellow, moves_yellow.to_vec());
            }
        }
    }
    (res, res_yellow)
}
