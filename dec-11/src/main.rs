use std::env;
use std::fs;
use std::cell::RefCell;

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
    const MONKEY_SIZE: usize = 7;

    use crate::part1::*;

    let mut monkeys: Vec<RefCell<Box<Monkey>>> = vec![];

    let lines: Vec<&str> = contents.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        println!("Line {}", i);
        let monkey_lines = &lines[i..i + MONKEY_SIZE - 1];
        let next_monkey = Monkey::parse_monkey(monkey_lines);
        monkeys.push(next_monkey);
        i += MONKEY_SIZE;
    }

    for round in 1..=4 {
        for m in &monkeys {
            let throws = m.borrow_mut().operate();
            for t in throws {
                let mut target = monkeys[t.0].borrow_mut();
                target.push_item(t.1);
            }
        }

        println!("After round {}, the monkeys are holding items with these worry levels:", round);
        for m in &monkeys {
            println!("{}", m.borrow());
        }
    }

    let mut inspection_counts: Vec<i32> = vec![];
    for m in &monkeys {
        let inspection_count = m.borrow().get_inspection_count();
        println!("Monkey {} inspected items {} times.", m.borrow().id, inspection_count);
        inspection_counts.push(inspection_count);
    }

    //reverse sort
    inspection_counts.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspection_counts[0] * inspection_counts[1];

    println!("Monkey Business value is: {}", monkey_business);
}

fn part2(contents: &String) {
    //NOTE: the only differences from part 1
    //are the number of rounds increased to 10000
    //and the size of the monkey business number is no
    //U1024 to hold a large enough number
    //slight changes from part1 module to part2 as well q.v.

    const MONKEY_SIZE: usize = 7;

    use crate::part2::*;

    let mut monkeys: Vec<RefCell<Box<Monkey>>> = vec![];

    let lines: Vec<&str> = contents.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        println!("Line {}", i);
        let monkey_lines = &lines[i..i + MONKEY_SIZE - 1];
        let next_monkey = Monkey::parse_monkey(monkey_lines);
        monkeys.push(next_monkey);
        i += MONKEY_SIZE;
    }

    let mut moduli: Vec<u64> = vec![];
    for m in &monkeys {
        moduli.push(m.borrow().get_modulus());
    }

    for m in &monkeys {
        m.borrow_mut().set_moduli(&moduli);
    }

    for round in 1..=10000 {
        for m in &monkeys {
            let throws = m.borrow_mut().operate();
            for t in throws {
                let mut target = monkeys[t.0].borrow_mut();
                target.push_item(t.1);
            }
        }

        println!("After round {}, the monkeys are holding items with these worry levels:", round);
        for m in &monkeys {
            println!("{}", m.borrow());
        }
    }

    let mut inspection_counts: Vec<u64> = vec![];
    for m in &monkeys {
        let inspection_count = m.borrow().get_inspection_count();
        println!("Monkey {} inspected items {} times.", m.borrow().id, inspection_count);
        inspection_counts.push(inspection_count);
    }

    //reverse sort
    inspection_counts.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspection_counts[0] * inspection_counts[1];

    println!("Monkey Business value is: {}", monkey_business);
}