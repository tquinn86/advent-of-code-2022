use std::env;
use std::fs;
use crate::position::Position;

pub mod position;
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

fn part1(contents: &String) {
    use crate::part1::{Grid, Blocker, SandUnit};

    let mut grid = Grid::parse_grid(contents);

    println!("{}", grid);

    let mut i = 0;
    'outer: loop {
        let mut su = SandUnit::new();

        while su.can_move(&grid) {
            su.do_move(&grid);

            if su.position().y() > grid.max_y() {
                break 'outer;
            }
        }
        grid.block(su.position(), Blocker::Sand);

        println!("{}", grid);
        i += 1
    }

    println!("Max sand units before abyss: {}", i);
}

fn part2(contents: &String) {
    use crate::part2::{Grid, Blocker, SandUnit};

    let mut grid = Grid::parse_grid(contents);

    println!("{}", grid);

    let mut i = 0;
    loop {
        let mut su = SandUnit::new();

        if grid.is_blocked(SandUnit::origin()) {
            break;
        }

        while su.can_move(&grid) {
            su.do_move(&grid);
        }
        grid.block(su.position(), Blocker::Sand);

        println!("{}", grid);
        i += 1
    }

    println!("Max sand units before blocking source: {}", i);
}

