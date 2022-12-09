use std::env;
use std::fs;

use crate::items::ItemTree;

pub mod items;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    let mut tree = ItemTree::new();
    let _ = tree.parse_commands(&contents);

    println!("{}", tree);

    let mut i = 0;
    let max = 100000;
    let mut total : u64 = 0;
    loop {
        let item = match tree.get_at(i) {
            Some(i) => i,
            None => break
        };

        if item.is_dir() {
            let dir_size = tree.get_item_size(i).unwrap();
            println!("Directory {} size is {}", item.name(), dir_size);
            if dir_size <= max && dir_size > 0 {
               //println!("Directory {} size is {}", item.name(), dir_size);
               total += dir_size;
            }
        }
        i += 1;
    }


    println!("Total of less that or equal to 100000: {}", total);
}

