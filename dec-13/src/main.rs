use std::env;
use std::fs;


pub mod part1;
//pub mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    part1(&contents);
    //part2(&contents);
}

fn part1(contents: &String) {

    use crate::part1::PairType;

    let lines: Vec<&str> = contents.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let p1 = PairType::parse_array(&lines[i]);
        let p2 = PairType::parse_array(&lines[i + 1]);
        i = i + 3
    }
}