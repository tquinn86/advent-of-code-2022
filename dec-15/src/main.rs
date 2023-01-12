use std::env;
use std::fs;
use crate::position::Position;
use crate::grid::Grid;

pub mod position;
pub mod grid;
pub mod part1;
//pub mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut line_number = 2000000;

    if let Some(ln) = args.get(2) {
        line_number = ln.parse::<i32>().unwrap();
    }

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    //part1(&contents, line_number);
    part2(&contents);
}

fn part1(contents: &String, line_number: i32) {

    use crate::part1::{Tile, Lock};

    let mut grid: Grid<Tile> = Grid::new();
    let mut locks: Vec<Lock> = vec![];

    for line in contents.lines() {
        grid.parse_line(line, &mut locks);
    }

    //println!("{:?}", grid);
    //println!("{}", grid);

    grid.fill_blocks_on_line(&locks, line_number);

    //println!("{:?}", grid);
    //println!("{}", grid);

    let blocks = grid.get_blocks_on_line(line_number);
    println!("Blocks on line: {}", blocks);
}

fn part2(contents: &String) {
    use std::collections::HashSet;

    let mut positions: HashSet<Position> = HashSet::new();

    for x in 0..4000000 {
        for y in 0..4000000 {
            let pos = Position::new(x, y);
            positions.insert(pos);
        }
    }
}