mod cube;
mod moves;
mod solver;

use cube::Cube;
use moves::parser::MoveParser;
use crate::moves::parse_file::open_file;
use crate::solver::create_reverse_file::create_reverse_file;
//use solver;
use crate::solver::solver::solver;
use crate::moves::print::transform_moves_to_notation;

// The main function is the entry point of the program.
// It parses command-line arguments, applies moves to the cube, and solves the cube.
fn main() {
    let (First_arg, move_sequence) = MoveParser::parse_arguments();

    if First_arg == moves::parser::First_arg::Generate {
        create_reverse_file();
        return;
    }

    match MoveParser::parse(&move_sequence, true) {
        Ok(moves) => {
            let mut cube = Cube::new();

            MoveParser::execute_moves(&mut cube, &moves, First_arg == moves::parser::First_arg::Print);
            println!("\nCalculating moves to solve the cube...");
            let (heuristic_table, heuristic_table_yellow) = open_file();
            println!("length: {}", heuristic_table.len());
            let solver = solver(&mut cube, heuristic_table, heuristic_table_yellow, true);
            println!("\nSolution:");
            let output = transform_moves_to_notation(&solver);
            println!("{}", output);
        }
        Err(err) => {
            eprintln!("Error parsing moves: {}", err);
        }
    }
}
