use crate::items_enum::*;
use std::rc::Rc;

pub fn part1(tree: &Rc<Item>) {

    let dirs = tree.flatten_dirs();
    let mut small_dirs: Vec<usize> = vec![];
    for d in *dirs {
        println!("Directory {} is of size {}", d.name(), d.size());

        if d.size() < 100000 { small_dirs.push(d.size()); }
    }

    println!("Small dirs add up to {}", small_dirs.iter().map(|s| s).sum::<usize>());
}

pub fn part2(tree: &Rc<Item>) {

    // Part 2 different calculation
    // find the smallest directory that will get us to > 30000000 free space
    // total space is 70000000. 7000000 - size of "/" is current free space

    let total =   70000000;
    let desired = 30000000;
    let cur_free = total - tree.size();
    println!("Current free space: {}", cur_free);
    let needed: usize = desired - cur_free;

    println!("Disk space needed: {}", needed);

    let mut candidates: Vec<usize> = vec![];
    let dirs = tree.flatten_dirs();
    for d in *dirs {
    
        let dir_size = d.size();
        if dir_size >= needed {
            println!("Directory {} size is {}", d.name(), dir_size);
            candidates.push(dir_size);
        } else {
            println!("Directory {} of size {} is not enough to free up {} space", d.name(), dir_size, needed);
        }
    }

    //sort and take the first (smallest) total
    candidates.sort();
    println!("Smallest directory that can free up enough space has a size of: {}", candidates[0]);
}