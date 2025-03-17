use crate::moves::parse_file::vec_move_to_string;
use std::fs::File;
use crate::Cube;
use crate::solver::algorythm::{a_star_optimize, ida_star};
use crate::solver::solver::generate_heuristic_table_optimize;
use std::io::Write;

const MAX_DEPTH_HEURISTIC: u8 = 7;
const MAX_DEPTH_OPTIMIZE: u8 = 10;

pub fn create_reverse_file()
{
    let random_cubes = Cube::generate_all_randoms_cube();
    println!("Random cubes generated, length: {}", random_cubes.len());
    let heuristic_table = generate_heuristic_table_optimize(Cube::get_edges_default(), MAX_DEPTH_HEURISTIC);
    let mut random_cubes = Cube::generate_all_randoms_cube();
    if !random_cubes.is_empty() {
        random_cubes.remove(0);
    }
    let mut file = File::create(&("./zbll-list/reverse-zbll.txt")).expect(
        "Failed to create file"
    );
    println!("Heuristic table generated, length: {}", heuristic_table.len());
    let mut i = 0;
    for random_cube in random_cubes {
        let mv = ida_star(&heuristic_table, random_cube,  Cube::get_edges_default(), MAX_DEPTH_OPTIMIZE, None);
        /*if !mv.1{
            println!("No solution found for cube : {}, number : {}", random_cube.to_string(), i);
            continue;
        }*/
        let line = match mv {
            Some(moves) => format!("{}: {}\n", random_cube.to_string(), vec_move_to_string(&moves)),
            None => format!("{}: No solution found\n", random_cube.to_string()),
        };
        file.write_all(line.as_bytes()).expect("Failed to write to file");
        i += 1;
    }
}