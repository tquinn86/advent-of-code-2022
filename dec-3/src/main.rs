use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    let mut total = 0;
    for line in contents.lines() {
        let c = get_common_char(line);
        let val = get_char_value(c);
        println!("{}:{}", c, val);
        total += val;
    }

    println!("Total priorities of misplaced items: {}", total);
}

fn get_common_char(line: &str) -> char {
    println!("{}", line);
    
    let halves = line.split_at(line.len() / 2);
    
    for i in 0..halves.0.len() {
        if halves.1.contains(halves.0.chars().nth(i).unwrap()) {
            return halves.0.chars().nth(i).unwrap();
        }
    }

    panic!("no matching char found.")
}

fn get_char_value(c: char) -> u32 {
    //need to return 1..26 for a..z and 27..52 for A..Z
    if c.is_ascii_lowercase() {
        return (c as u32) - 96;
    } else if c.is_ascii_uppercase() {
        return (c as u32) - 38;
    } else {
        panic!("Invalid char")
    }
}