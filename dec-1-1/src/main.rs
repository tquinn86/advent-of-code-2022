use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect("Cannot find file {}");

    //println!("{contents}");

    let mut elf = 0;
    let mut elf_cal = 0;
    let mut v: Vec<(i32, i32)> = Vec::new();

    for line in contents.lines() {  
        if line == "" {
            //println! ( "Elf {} has {} calories of food.", elf + 1, elf_cal);
            v.push((elf + 1, elf_cal));
            elf = elf + 1;
            elf_cal = 0;
        }
        else {
            elf_cal += line.parse::<i32>().unwrap();
        }
    }

    v.sort_by(|a, b| b.1.cmp(&a.1));

    let mut top3_total = 0;
    for x in 0..3 { 
        println!("Elf {} has {} cals.", v[x].0, v[x].1);
        top3_total += v[x].1;
    }

    println!("Top 3 elves total {} cals.", top3_total);
}
