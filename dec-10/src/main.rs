use std::env;
use std::fs;

pub mod sprite; //for part2

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

    let mut mark_point = 20;
    let mark_point_inc = 40;

    let mut markers: Vec<(i32,i32)> = vec![];

    let mut cycle = 1;
    let mut x_reg = 1;

    let mut record_mark = |c, x| {
        if c == mark_point {
            println!("Mark point {}, x register = {}, signal strenght = {}", c, x, c * x);
            mark_point += mark_point_inc;
            markers.push((c, c * x));
        }
    };

    for line in contents.lines() {

        if line == "noop" {
            record_mark(cycle, x_reg);
            cycle += 1;
        } else if line.starts_with("addx") {
            let add_val: i32  = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            for _ in 0..2
            {
                record_mark(cycle, x_reg);
                cycle += 1;
            }
            x_reg += add_val
        } else {
            panic!("Unknown instruction")
        }
    }

    println!("Sum of mark points is {}", markers.iter().map(|t| t.1).sum::<i32>());
}

fn part2(contents: &String) {

    use crate::sprite::Sprite;

    let mut mark_point = 39;
    let mark_point_inc = 40;

    let mut cycle = 0;
    let mut sprite: Sprite = Sprite::new(1);
    let mut line_number = 0;

    let mut record_mark = |c, l: &mut i32| {
        if c == mark_point {
            println!();
            mark_point += mark_point_inc;
            *l += 1;
        }
    };

    for line in contents.lines() {

        if line == "noop" {
            sprite.draw(cycle - (line_number * mark_point_inc));
            record_mark(cycle, &mut line_number);
            cycle += 1;
        } else if line.starts_with("addx") {
            let add_val: i32  = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            for _ in 0..2
            {
                sprite.draw(cycle - (line_number * mark_point_inc));
                record_mark(cycle, &mut line_number);
                cycle += 1;
            }
            sprite += add_val
        } else {
            panic!("Unknown instruction")
        }
    }
}