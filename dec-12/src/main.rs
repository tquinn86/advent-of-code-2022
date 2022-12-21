use std::env;
use std::fs;


pub mod part1;
pub mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String)
{
    use crate::part1::Grid;

    let grid = Grid::parse_grid(contents);

    grid.print_grid_chars();

    grid.build_tree();

    println!("Shortest distance from S to E is: {}", grid.calc_shortest_path());
}

fn part2(contents: &String)
{
    use crate::part2::{Grid, Position};

    let base = Grid::parse_grid(contents);

    base.print_grid_chars();

    //loop through the grid and if the value is "1" (a) 
    //then clone the grid with that position as start, build the tree
    //and get the distance
    let mut distances: Vec<i32> = vec![];

    for y in 0..=base.max_position.y {
        for x in 0..=base.max_position.x {
            let p = Position{ x: x, y: y };
            let v = base.get_value(p);
            if v == 1 {
                let mut grid_new = base.clone_with_new_start(p);
                grid_new.build_tree();
                let distance = grid_new.calc_shortest_path();
                match distance {
                    Err(s) => {
                        println!("{}", s);
                        println!("Cannot get from {:?} to E", p);
                    },
                    Ok(d) => {
                        distances.push(d);
                        println!("Shortest distance from {:?} to E is: {:?}", p, d);
                    }
                }
            }
        }
    }

    distances.sort();

    println!("Shortest of all: {}", distances[0]);
}
