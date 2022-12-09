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

    // Part 1
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

    // Part 2 different calculation
    // find the smallest directory that will get us to > 30000000 free space
    // total space is 70000000. 7000000 - size of "/" is current free space

    let total =   70000000;
    let desired = 30000000;
    let cur_free = total - tree.get_item_size(0).unwrap();
    println!("Current free space: {}", cur_free);
    let needed = desired - cur_free;

    println!("Disk space needed: {}", needed);

    let mut candidates: Vec<u64> = vec![];

    i = 0;
    loop {
        let item = match tree.get_at(i) {
            Some(i) => i,
            None => break
        };

        if item.is_dir() {
            let dir_size = tree.get_item_size(i).unwrap();
            if dir_size >= needed {
               println!("Directory {} size is {}", item.name(), dir_size);
               candidates.push(dir_size);
            } else {
                println!("Directory {} of size {} is not enough to free up {} space", item.name(), dir_size, needed);
            }
        }
        i += 1;
    }

    //sort and take the first (smallest) total
    candidates.sort();
    println!("Smallest directory that can free up enough space has a size of: {}", candidates[0]);
}

