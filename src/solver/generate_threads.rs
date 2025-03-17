use crate::cube::Cube;
use crate::moves::movement::Move;
use crate::cube::color::OneColor;
use crate::solver::solver::{execute_algorithm_heuristic, generate_heuristic_table};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool};
use std::thread;

use std::sync::Arc;

pub fn generate_thread_last_edges(cube: &mut Cube) -> Vec<Move> {
    let colors = vec![
        vec![],
        vec![OneColor::Yellow, OneColor::Green],
        vec![OneColor::Yellow, OneColor::Red],
        vec![OneColor::Yellow, OneColor::Orange],
        vec![OneColor::Yellow, OneColor::Blue],
        vec![OneColor::Yellow, OneColor::Orange, OneColor::Green],
        vec![OneColor::Yellow, OneColor::Green, OneColor::Red],
        vec![OneColor::Yellow, OneColor::Red, OneColor::Blue],
        vec![OneColor::Yellow, OneColor::Blue, OneColor::Orange],
    ];

    let names = vec![
        "default".to_string(),
        "green edges".to_string(),
        "red edges".to_string(),
        "orange edges".to_string(),
        "blue edges".to_string(),
        "orange green edges".to_string(),
        "green red edges".to_string(),
        "red blue edges".to_string(),
        "blue orange edges".to_string()
    ];

    let result = generate_thread(
        names.clone(),
        colors.clone(),
        cube,
        Cube::get_two_yellow_edges_default,
        Cube::get_two_yellow_edges_cube_static
    );
    println!("Last edges solution: ");
    cube.print();
    result.0
}

pub fn generate_thread_white_corner(cube: &mut Cube) -> Vec<Move> {

    let colors = vec![
        vec![],
        vec![OneColor::Red, OneColor::Green, OneColor::White],
        vec![OneColor::Green, OneColor::Orange, OneColor::White],
        vec![OneColor::Orange, OneColor::Blue, OneColor::White],
        vec![OneColor::Blue, OneColor::Red, OneColor::White],
        vec![OneColor::Red, OneColor::Green, OneColor::White, OneColor::Blue],
        vec![OneColor::Green, OneColor::Orange, OneColor::White, OneColor::Red],
        vec![OneColor::Orange, OneColor::Blue, OneColor::White, OneColor::Green],
        vec![OneColor::Blue, OneColor::Red, OneColor::White, OneColor::Orange],
    ];

    let names = vec![
        "default".to_string(),
        "green corner".to_string(),
        "red corner".to_string(),
        "blue corner".to_string(),
        "orange corner".to_string(),
        "green blue corner".to_string(),
        "orange red corner".to_string(),
        "blue orange corner".to_string(),
        "red green corner".to_string()
    ];

    let result = generate_thread(names, colors, cube, Cube::get_three_white_corner_default, Cube::get_three_white_corner_cube_static);
    println!("White corner solution: ");
    cube.print();
    return result.0;
}

pub fn generate_thread_white_corner_good_place(cube: &mut Cube) -> Vec<Move> {
    let colors = vec![
        vec![],
        vec![OneColor::Red, OneColor::Green, OneColor::White],
        vec![OneColor::Green, OneColor::Orange, OneColor::White],
        vec![OneColor::Orange, OneColor::Blue, OneColor::White],
        vec![OneColor::Blue, OneColor::Red, OneColor::White],
        vec![OneColor::Red, OneColor::Green, OneColor::Blue, OneColor::White],
        vec![OneColor::Green, OneColor::Orange, OneColor::Red, OneColor::White],
        vec![OneColor::Orange, OneColor::Blue, OneColor::Green, OneColor::White],
        vec![OneColor::Blue, OneColor::Red, OneColor::Orange, OneColor::White],
    ];

    let names = vec![
        "default".to_string(),
        "green corner".to_string(),
        "red corner".to_string(),
        "blue corner".to_string(),
        "orange corner".to_string(),
        "green blue corner".to_string(),
        "orange red corner".to_string(),
        "blue orange corner".to_string(),
        "red green corner".to_string()
    ];

    let result = generate_thread(names, colors, cube, Cube::get_good_place_corner_default, Cube::get_good_place_corner_cube_static);
    println!("White corner solution: ");
    cube.print();
    return result.0;
}

pub fn generate_thread<F, G>(names: Vec<String>, colors: Vec<Vec<OneColor>>, cube: &mut Cube, generate_cube_default: F, generate_cube_static: G) -> (Vec<Move>, bool)
where
    F: Fn(&[OneColor]) -> Cube + std::marker::Send,
    G: Fn(Cube, &[OneColor]) -> Cube + std::marker::Send,
{
    let mut heuristics_threads = colors
        .iter()
        .map(|color_set| {
            let new_cube = generate_cube_default(&color_set);
            thread::spawn(move || generate_heuristic_table(new_cube, 4))
        })
        .collect::<Vec<_>>();

    let _solution: Vec<Move> = vec![];
    let _find = false;

    let mut heuristic_tables_mut = Vec::<HashMap<Cube, u8>>::new();
    for (_i, _color_set) in colors.iter().enumerate() {
        let thread = heuristics_threads.remove(0);
        let result = thread.join().unwrap();
        heuristic_tables_mut.push(result);
    }

    /*for heuristic_table in heuristic_tables_mut.iter() {
        println!("Heuristic table: ");
        if let Some((key, value)) = heuristic_table.iter().next() {
        key.print();
        }
    }*/

    let heuristic_tables = Arc::new(heuristic_tables_mut);
    let stop_flag = Arc::new(AtomicBool::new(false));

    let mut algorythm_threads = colors
        .iter()
        .enumerate()
        .map(|(i, color_set)| {
            let new_color_set = color_set.clone();
            let new_name = names[i].clone();
            let stop_flag_clone = Arc::clone(&stop_flag);
            let mut new_cube = cube.clone();
            let static_cube = generate_cube_static(new_cube.clone(), &new_color_set);
            let heuristic_tables_clone = Arc::clone(&heuristic_tables);
            thread::spawn(move || {
                let tables = heuristic_tables_clone;
                execute_algorithm_heuristic(
                    &mut new_cube,
                    &new_name,
                    static_cube,
                    6,
                    &tables[i],
                    Some(&stop_flag_clone),
                    false,
                )
            })
        })
        .collect::<Vec<_>>();

    let mut res: Vec<(String, Vec<Move>, bool)> = Vec::new();
    let mut i = 0;
    while algorythm_threads.len() > 0 {
        let thread = algorythm_threads.remove(0);
        let (solution, find) = thread.join().unwrap();
        res.push((names[i].clone(), solution, find));
        i += 1;
    }
    
    let mut solutions_found = Vec::new();

    for (name, solution, find) in res.iter() {
        if *find {
            if name == "default" {
                for mv in solution.iter() {
                    cube.apply_move(mv.face, mv.rotation);
                }
                return (solution.clone(), find.clone());
            }
            else {
                let mut ncube = cube.clone();
                for mv in solution.iter() {
                    ncube.apply_move(mv.face, mv.rotation);
                }
                println!("Solution found for {}", name);
                if name == "green corner" {
                    let mut nncube = cube.clone();
                    for mv in solution.iter() {
                        nncube.apply_move(mv.face, mv.rotation);
                    }
                    nncube.print();
                }
                solutions_found.push(solution);
            }
        }
    }

    println!("first solution not found, searching with {} threads", solutions_found.len());

    algorythm_threads = solutions_found.iter().enumerate().map(|(i, solution)| {
        let new_color_set = colors[0].clone();
        let new_name = names[0].clone();
        let stop_flag_clone = Arc::clone(&stop_flag);
        let mut new_cube = cube.clone();
        for mv in solution.iter() {
            new_cube.apply_move(mv.face, mv.rotation);
        }
        let static_cube = generate_cube_static(new_cube.clone(), &new_color_set);
        let heuristic_tables_clone = Arc::clone(&heuristic_tables);
        thread::spawn(move || {
            let tables = heuristic_tables_clone;
            execute_algorithm_heuristic(&mut new_cube, &new_name, 
                static_cube,
                6, 
                &tables[0],
                Some(&stop_flag_clone),
                false
            )
        })
    }).collect::<Vec<_>>();

    let mut second_res = Vec::new();
    i = 0;
    while algorythm_threads.len() > 0 {
        let thread = algorythm_threads.remove(0);
        let (solution, find) = thread.join().unwrap();
        second_res.push((names[i].clone(), solution, find));
        i += 1;
    }

    i = 0;
    for (name, solution, find) in second_res.iter() {
        if *find {
            println!("Solution found for {}", name);
            for mv in solutions_found[i].iter() {
                cube.apply_move(mv.face, mv.rotation);
            }
            for mv in solution.iter() {
                cube.apply_move(mv.face, mv.rotation);
            }
            cube.print();
            return (vec![solutions_found[i].clone(), solution.clone()].concat(), find.clone());
        }
        i += 1;
        println!("not found...");
    }
    println!("No solution found");

    (vec![], false)
}
