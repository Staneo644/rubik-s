use crate::solver::solver::MOVES;
use crate::cube::Cube;
use crate::moves::movement::Move;
use crate::solver::EFace;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering as CmpOrdering;
use crate::cube::optimise_cube::Optimise_cube;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Eq, PartialEq)]
struct Node {
    cube: Cube,
    g_score: u8,
    last_face: EFace,
    moves: Vec<Move>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        (other.g_score).cmp(&(self.g_score))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

fn find_heuristic(heuristic_table: &HashMap<Cube, u8>, cube: &mut Cube)
     -> Vec<Move> {
    let mut moves = Vec::new();
    let mut current_h = *heuristic_table.get(cube).unwrap_or(&u8::MAX);
    while current_h > 0
    {
        for mv in MOVES.iter() {
            let mut new_cube = cube.clone();
            new_cube.apply_move(mv.face, mv.rotation);
            let new_h = *heuristic_table.get(&new_cube).unwrap_or(&u8::MAX);
            if new_h < current_h
            {
                current_h = new_h;
                moves.push(mv.clone());
                cube.apply_move(mv.face, mv.rotation);
                break;
            }
        }
    }
    moves
}

pub fn a_star(heuristic_table: &HashMap<Cube, u8>, start: Cube, max_exploration: u8, stop_flag: Option<&AtomicBool>)
     -> (Vec<Move>, bool) {
    let mut open_set = BinaryHeap::new();
    let mut visited = HashMap::new();
    let h_start = *heuristic_table.get(&start).unwrap_or(&u8::MAX);
    let mut g_level = 0;
    if h_start != u8::MAX
    {
        return (find_heuristic(heuristic_table, &mut start.clone()), true);
    }
    open_set.push(Node {
        cube: start.clone(),
        g_score: 0,
        last_face: EFace::Up,
        moves: Vec::new(),
    });

    println!("heuristic table size: {}", heuristic_table.len());

    if !stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
        return (Vec::new(), false);
    }
    while let Some(Node {cube, g_score, last_face, moves}) = open_set.pop() {
        let new_g = g_score + 1;
        if new_g > g_level {
            if new_g == max_exploration || !stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
                return (Vec::new(), false);
            }
            g_level = new_g;
        }

        for mv in MOVES.iter() {
            if last_face == mv.face && g_score > 0 {
                continue;
            }
            let mut new_cube = cube.clone();
            new_cube.apply_move(mv.face, mv.rotation);
            if let Some(&best_g) = visited.get(&new_cube) {
                continue;
            }
            visited.insert(cube.clone(), g_score);
            let new_h = *heuristic_table.get(&new_cube).unwrap_or(&u8::MAX);
            let mut new_moves = moves.clone();
            new_moves.push(mv.clone());

            if new_h != u8::MAX
            {
                return (
                    vec![
                        new_moves, 
                        find_heuristic(heuristic_table, &mut new_cube)
                    ].concat(),
                    true
                );
            }
            open_set.push(Node {
                cube: new_cube,
                g_score: new_g,
                last_face: mv.face,
                moves: new_moves,
            });
        }
    }
    println!("no solution found, open set length: {}", open_set.len());
    (Vec::new(), false)
}

fn find_heuristic_optimize(heuristic_table: &HashMap<Optimise_cube, u8>, cube: &mut Cube)
     -> Vec<Move> {
    let mut moves = Vec::new();
    let mut current_h = *heuristic_table.get(&Optimise_cube::new(&cube)).unwrap_or(&u8::MAX);
    while current_h > 0
    {
        for mv in MOVES.iter() {
            let mut new_cube = cube.clone();
            new_cube.apply_move(mv.face, mv.rotation);
            let new_h = *heuristic_table.get(&Optimise_cube::new(&new_cube)).unwrap_or(&u8::MAX);
            if new_h < current_h
            {
                current_h = new_h;
                moves.push(mv.clone());
                cube.apply_move(mv.face, mv.rotation);
                break;
            }
        }
    }
    moves
}

pub fn a_star_optimize(heuristic_table: &HashMap<Optimise_cube, u8>, start: Cube, max_exploration: u8, stop_flag: Option<&AtomicBool>)
     -> (Vec<Move>, bool) {
    let mut open_set = BinaryHeap::new();
    let mut visited = HashMap::new();
    let h_start = *heuristic_table.get(&Optimise_cube::new(&start)).unwrap_or(&u8::MAX);
    let mut g_level = 0;
    if h_start != u8::MAX
    {
        return (find_heuristic_optimize(heuristic_table, &mut start.clone()), true);
    }
    open_set.push(Node {
        cube: start.clone(),
        g_score: 0,
        last_face: EFace::Up,
        moves: Vec::new(),
    });

    if !stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
        return (Vec::new(), false);
    }
    while let Some(Node {cube, g_score, last_face, moves}) = open_set.pop() {
        if let Some(&best_g) = visited.get(&cube) {
            if g_score >= best_g {
                continue;
            }
        }
        visited.insert(cube.clone(), g_score);

        for mv in MOVES.iter() {
            if last_face == mv.face && g_score > 0 {
                continue;
            }
            let mut new_cube = cube.clone();
            new_cube.apply_move(mv.face, mv.rotation);
            let new_g = g_score + 1;
            if new_g > g_level {
                if new_g == max_exploration || !stop_flag.map_or(true, |flag| !flag.load(Ordering::Relaxed)) {
                    return (Vec::new(), false);
                }
                println!("g level: {}", new_g);
                g_level = new_g;
            }
            visited.insert(cube.clone(), g_score);
            let new_h = *heuristic_table.get(&Optimise_cube::new(&new_cube)).unwrap_or(&u8::MAX);
            let mut new_moves = moves.clone();
            new_moves.push(mv.clone());

            if new_h != u8::MAX
            {
                return (
                    vec![
                        new_moves, 
                        find_heuristic_optimize(heuristic_table, &mut new_cube)
                    ].concat(),
                    true
                );
            }
            open_set.push(Node {
                cube: new_cube,
                g_score: new_g,
                last_face: mv.face,
                moves: new_moves,
            });
        }
    }
    println!("no solution found, open set length: {}", open_set.len());
    (Vec::new(), false)
}
pub fn ida_star(heuristic_table: &HashMap<Optimise_cube, u8>, cube: Cube, solved_cube: Cube, default_heuristic: u8, stop_flag: Option<&AtomicBool>) -> Option<Vec<Move>> {
    fn search(
        node: &Cube,
        solved_cube: &Cube,
        last_face: Option<EFace>,
        g_score: u8,
        threshold: u8,
        heuristic_table: &HashMap<Optimise_cube, u8>,
        default_heuristic: u8,
    ) -> Result<Vec<Move>, u8> {
        let state_key = &node;
        let h_score = *heuristic_table.get(&Optimise_cube::new(&node)).unwrap_or(&default_heuristic);
        /*if h_score != default_heuristic {
            println!("Heuristic: {} moves: {}", h_score, g_score);
        }*/
        let f_score = g_score + h_score;

        if node == solved_cube {
            return Ok(Vec::new());
        }
        if f_score > threshold {
            return Err(f_score);
        }
        
        let mut min_threshold = u8::MAX;
        
        for mv in MOVES.iter() {
            match last_face {
                Some(face) if face == mv.face => continue,
                _ => (),
            }
            let mut new_cube = node.clone();
            new_cube.apply_move(mv.face, mv.rotation);
            match search(&new_cube, &solved_cube, Some(mv.face), g_score + 1, threshold, heuristic_table, default_heuristic) {
                Ok(mut path) => {
                    path.insert(0, mv.clone());
                    return Ok(path);
                }
                Err(new_threshold) => {
                    if new_threshold < min_threshold {
                        min_threshold = new_threshold;
                    }
                }
            }
        }
        
        Err(min_threshold)
    }

    let mut threshold = *heuristic_table.get(&Optimise_cube::new(&cube)).unwrap_or(&default_heuristic);
    loop {
        match search(&cube, &solved_cube, None, 0, threshold, heuristic_table, default_heuristic) {
            Ok(solution) => return Some(solution),
            Err(new_threshold) => {
                if new_threshold == u8::MAX {
                    return None;
                }
                threshold = new_threshold;
            }
        }
    }
}