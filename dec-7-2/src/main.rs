use std::env;
use std::fs;
use std::rc::Rc;

//use crate::items_enum::commands::*;
//use crate::items_enum::Item;
//use crate::items_enum::problem::* ; 

use crate::items_trait::commands::*;
use crate::items_trait::Item;
use crate::items_trait::problem::* ; 

pub mod items_enum;
pub mod items_trait;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    let tree = parse_commands(&contents);

    println!("{}", tree);

    part1(&tree);
    part2(&tree);
}

