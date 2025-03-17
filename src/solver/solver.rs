use std::collections::{ HashMap, VecDeque };
use std::thread::JoinHandle;
use std::thread;
use crate::cube::Cube;
use crate::moves::movement::Move;
use crate::moves::parse_file::vec_move_to_string;
use crate::cube::EFace;
use crate::solver::algorythm::{ a_star, a_star_optimize };
use crate::solver::generate_threads::{ generate_thread_last_edges, generate_thread_white_corner, generate_thread_white_corner_good_place };
use crate::cube::color::OneColor;
use crate::cube::optimise_cube::Optimise_cube;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::io::Write;
use std::sync::atomic::Ordering;

pub const MOVES: [Move; 18] = [
    Move { face: EFace::Front, rotation: 1 },
    Move { face: EFace::Front, rotation: -1 },
    Move { face: EFace::Front, rotation: 2 },

    Move { face: EFace::Down, rotation: 1 },
    Move { face: EFace::Down, rotation: -1 },
    Move { face: EFace::Down, rotation: 2 },

    Move { face: EFace::Left, rotation: 1 },
    Move { face: EFace::Left, rotation: -1 },
    Move { face: EFace::Left, rotation: 2 },

    Move { face: EFace::Right, rotation: 1 },
    Move { face: EFace::Right, rotation: -1 },
    Move { face: EFace::Right, rotation: 2 },

    Move { face: EFace::Up, rotation: 1 },
    Move { face: EFace::Up, rotation: -1 },
    Move { face: EFace::Up, rotation: 2 },

    Move { face: EFace::Bottom, rotation: 1 },
    Move { face: EFace::Bottom, rotation: -1 },
    Move { face: EFace::Bottom, rotation: 2 },
];

pub fn generate_heuristic_table(solved_cube: Cube, max_depth: u8) -> HashMap<Cube, u8> {
    let mut heuristic_table = HashMap::new();
    let mut queue: VecDeque<(Optimise_cube, u8, EFace)> = VecDeque::new();

    queue.push_back((Optimise_cube::new(&solved_cube), 0, EFace::Up));

    while let Some((optimise_cube, depth, face)) = queue.pop_front() {
        if depth > max_depth {
            break;
        }
        let mut state_key = Cube::new_optimized(&optimise_cube);

        if !heuristic_table.contains_key(&state_key) {
            heuristic_table.insert(state_key, depth);

            for mv in MOVES.iter() {
                if face == mv.face && depth > 0 {
                    continue;
                }
                state_key.apply_move(mv.face, mv.rotation);

                //let mut new_cube = state_key.clone();
                //new_cube.apply_move(mv.face, mv.rotation);

                queue.push_back((Optimise_cube::new(&state_key), depth + (1 as u8), mv.face));
                state_key.apply_move(mv.face, match mv.rotation {
                    1 => -1,
                    -1 => 1,
                    2 => 2,
                    _ => 0
                });
            }
        }
    }
    heuristic_table
}

pub fn generate_heuristic_table_optimize(solved_cube: Cube, max_depth: u8) -> HashMap<Optimise_cube, u8> {
    let mut heuristic_table = HashMap::new();
    let mut queue: VecDeque<(Optimise_cube, u8, EFace)> = VecDeque::new();
    let mut actual_depth = 0;

    queue.push_back((Optimise_cube::new(&solved_cube), 0, EFace::Up));

    while let Some((optimise_cube, depth, face)) = queue.pop_front() {
        if depth > actual_depth {
            let total_size_allocation_vec = queue.len() * size_of::<(Optimise_cube, u8, EFace)>();
            let to_giga_octet = total_size_allocation_vec as f64 / 1024.0 / 1024.0 / 1024.0;
            println!("Actual depth: {}, queue size: {}, total size allocation: {}, to giga octet: {}", actual_depth, queue.len(), total_size_allocation_vec, to_giga_octet);
            actual_depth = depth;
        }
        if depth > max_depth {
            break;
        }
        let mut state_key = Cube::new_optimized(&optimise_cube);

        if !heuristic_table.contains_key(&optimise_cube) {
            heuristic_table.insert(optimise_cube.clone(), depth);

            for mv in MOVES.iter() {
                if face == mv.face && depth > 0 {
                    continue;
                }
                state_key.apply_move(mv.face, mv.rotation);

                queue.push_back((Optimise_cube::new(&state_key), depth + 1, mv.face));
                state_key.apply_move(mv.face, match mv.rotation {
                    1 => -1,
                    -1 => 1,
                    2 => 2,
                    _ => 0
                });
            }
        }
    }
    heuristic_table
}

pub fn execute_algorithm_heuristic(
    cube: &mut Cube,
    actual_operation: &str,
    filtered_cube: Cube,
    max_exploration: u8,
    heuristic_table: &HashMap<Cube, u8>,
    stop_flag: Option<&AtomicBool>,
    print: bool
) -> (Vec<Move>, bool) {
    let (solution, find) = a_star(&heuristic_table, filtered_cube, max_exploration, stop_flag);
    if !find || !stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
        return (Vec::new(), false);
    }
    if print {
        println!("Solution from {} found. Length: {}", actual_operation, solution.clone().len());
    }
    for mv in solution.clone() {
        cube.apply_move(mv.face, mv.rotation);
    }
    if print && stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
        println!("Cube after {}:", actual_operation);
        cube.print();
    }
    (solution, find)
}

pub fn execute_algorithm(
    cube: &mut Cube,
    actual_operation: &str,
    filtered_cube: Cube,
    max_exploration: u8,
    heuristic_thread: JoinHandle<HashMap<Cube, u8>>,
    print: bool
) -> (Vec<Move>, bool) {
    if print {
        println!("Actual goal from {}:", actual_operation);
        filtered_cube.print();
        println!("Waiting for heuristic table from {}...", actual_operation);
    }
    let heuristic_table = heuristic_thread.join().unwrap();
    if print {
        println!("Heuristic table from {} generated. Waiting for ida...", actual_operation);
    }
    return execute_algorithm_heuristic(
        cube,
        actual_operation,
        filtered_cube,
        max_exploration,
        &heuristic_table,
        None,
        print
    );
}

pub fn solver(
    cube: &mut Cube,
    heuristic_table_zbll: HashMap<Cube, Vec<Move>>,
    heuristic_table_yellow: HashMap<Cube, Vec<Move>>,
    print: bool
) -> Vec<Move> {
    let mut res = Vec::new();
    let mut solution;

    /*let _heuristic_one_white_corner = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_one_white_corner_default());
        heuristic_table
    });

    let _heuristic_two_white_corner = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_two_white_corner_default());
        heuristic_table
    });*/
    const heuristic_depth: u8 = 4;

    let heuristic_last_white_corner = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_last_white_corner_default(), heuristic_depth);
        heuristic_table
    });

    let heuristic_table = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::new(), heuristic_depth);
        heuristic_table
    });

    let heuristic_first_layer = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_first_layer_default(), heuristic_depth);
        heuristic_table
    });

    let heuristic_white_cross = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_white_cross_default(), heuristic_depth);
        heuristic_table
    });

    let heuristic_edges = thread::spawn(|| {
        let heuristic_table = generate_heuristic_table(Cube::get_edges_default(), heuristic_depth);
        heuristic_table
    });

    (solution, _) = execute_algorithm(
        cube,
        "white cross",
        cube.get_white_cross_cube(),
        10,
        heuristic_white_cross,
        print
    );
    res.extend(solution);
    
    /*solution = generate_thread_white_corner_good_place(cube);
    res.extend(solution);   

    cube.refresh_bottom_cube().print();
    let element = heuristic_table_yellow.get(&cube.refresh_bottom_cube());
    if element.is_some() {
        println!("Solution found for yellow");
        println!("{}", cube.to_string());
        for mv in element.unwrap().iter() {
            cube.apply_move(mv.face, mv.rotation);
        }
        res.extend(element.unwrap().clone());
        println!("{}", cube.to_string());
    }
    else {
        //println!("{}", cube.refresh_bottom_cube().to_string());
        println!("No solution found for yellow");
    }

    let element = heuristic_table_zbll.get(&cube.clone());
    if element.is_some() {
        println!("Solution found for zbll");
        println!("{}", cube.to_string());
        for mv in element.unwrap().iter() {
            cube.apply_move(mv.face, mv.rotation);
        }
        res.extend(element.unwrap().clone());
        println!("{}", cube.to_string());
    } else {
        println!("No solution found for zbll");
        println!("{}", cube.to_string());
        (solution, _) = execute_algorithm(
            cube,
            "solved cube",
            cube.clone(),
            10,
            heuristic_table,
            print
        );
        res.extend(solution);
    }*/

    println!("Total solution length: {}", res.len());
    res
}
