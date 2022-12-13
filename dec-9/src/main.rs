use std::env;
use std::fs;

pub mod rope_part1;
pub mod rope_part2;

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

    use crate::rope_part1::{Rope, Direction};
    
    let mut r = Rope::new();

    for line in contents.lines() {
        let d: char = line.chars().nth(0).unwrap();
        let steps: i32  = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        r.move_head(Direction::from_char(d), steps);
    }

    println!("Tail has occupied {} spots.", r.get_tail_history_count());
}

fn part2(contents: &String) {

    use crate::rope_part2::{Rope, Direction};
    
    let mut r = Rope::new(10);

    for line in contents.lines() {
        let d: char = line.chars().nth(0).unwrap();
        let steps: i32  = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

        r.move_head(Direction::from_char(d), steps);
    }

    println!("Tail has occupied {} spots.", r.get_tail_history_count());
}

