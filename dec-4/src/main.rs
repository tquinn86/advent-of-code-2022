use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    let mut total = 0;
    for line in contents.lines() {
        let areas = parse_line(line);

        total+= contains_areas(areas.0, areas.1);
    }
    println!("Total overlaps: {}", total);
}

fn parse_line(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let halves: Vec<&str> = line.split(',').collect();
    (parse_range(halves[0]), parse_range(halves[1]))
}

fn parse_range(range: &str) -> HashSet<i32> {
    let bounds: Vec<i32> = range.split('-').map(|a| a.parse::<i32>().unwrap()).collect();
    (bounds[0]..=bounds[1]).collect()
}

fn contains_areas(area1: HashSet<i32>, area2: HashSet<i32>) -> i32 {
    if area1.len() >= area2.len() {
        if area1.intersection(&area2).collect::<Vec<&i32>>().len() == area2.len() { 
            return 1;
        }
    }
    else
    {
        if area2.intersection(&area1).collect::<Vec<&i32>>().len() == area1.len() {
            return 1;
        } else {
            return 0;
        }
    }
    return 0;
}