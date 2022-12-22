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

fn part1(contents: &String) {

    use crate::part1::PairType;

    let lines: Vec<&str> = contents.lines().collect();

    let mut i = 0;
    let mut pairs: Vec<(PairType, PairType)> = vec![];
    while i < lines.len() {
        let p1 = PairType::parse_array(&lines[i]);
        let p2 = PairType::parse_array(&lines[i + 1]);
        pairs.push((p1.1, p2.1));
        i = i + 3
    }

    let mut good: Vec<usize> = vec![];
    for i in 0..pairs.len() {
        println!("{}", pairs[i].0);
        println!("{}", pairs[i].1);
        println!();

        if pairs[i].0 <= pairs[i].1 {
            println!("Pair {} is properly ordered", i + 1);
            good.push(i + 1);
        } else {
            println!("Pair {} is not properly ordered", i + 1);
        }
    }

    println!("Sum of the good pair indices is: {}", good.iter().sum::<usize>())
}

fn part2(contents: &String) {

    use crate::part2::PacketType;

    let mut codes: Vec<PacketType> = vec![];

    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }

        codes.push(PacketType::parse_array(&line).1);
    }

    //add the two special packets
    let code1 = PacketType::parse_array("[[2]]").1;
    let code2 = PacketType::parse_array("[[6]]").1;
    codes.push(code1);
    codes.push(code2);

    codes.sort();

    let code1 = PacketType::parse_array("[[2]]").1;
    let code2 = PacketType::parse_array("[[6]]").1;

    let mut code1_id: usize = 0;
    let mut code2_id: usize = 0;

    for i in 0..codes.len() {
        println!("{}", codes[i]);
        if codes[i] == code1 {
            code1_id = i + 1;
        }
        if codes[i] == code2 {
            code2_id = i + 1;
        }
    }

    println!("Product of decoder indices is: {}", code1_id * code2_id);
}