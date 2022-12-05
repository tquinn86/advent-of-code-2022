use std::env;
use std::fs;
use std::fmt;
use std::collections::VecDeque;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    let results = parse_file(&contents.lines().collect());
    let mut stacks = results.0;

    perform_moves(&results.1, &mut stacks);

    for i in 0..stacks.len() {
        print_stack(i, &stacks[i]);
    }
}

struct Move {
    num: i32,
    source: usize,
    target: usize
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move: number = {}, source stack {}, target stack {}", self.num, self.source, self.target)
    }
}

fn parse_file(lines: &Vec<&str>) -> (Vec<VecDeque<char>>, Vec<Move>) {
    //first we have to separate the file into stacks
    //and move instructions.

    //the first blank line will delineate the sections, we'll move the lines up to then
    //into a new vector that puts them in reverse order, it will be easier to parse that way.

    let mut stacks = VecDeque::<&str>::new();

    let mut iter = lines.iter();
    let mut line = iter.next();
    while *(line.unwrap()) != "" {
        println!("{}:{}", line.unwrap(), line.unwrap().len());
        stacks.push_front(line.unwrap());
        line = iter.next();
    }

    let parsed_stacks = parse_stacks(&mut stacks);

    //skip the blank line
    line = iter.next();
    let mut moves = Vec::<Move>::new();
    loop{
        let linestr: &str;
    
        match line {
            Some(thisstr) => linestr = thisstr,
            None => break
        }
        println!("{}", linestr);

        let m = parse_move(linestr);
        println!("{}", m);
        moves.push(m);

        line = iter.next();
    }
    (parsed_stacks, moves)
}

fn parse_stacks(stacks_raw: &mut VecDeque<&str>) -> Vec<VecDeque<char>>
{
    //pop the first line, this tells us how many stacks we have
    let stack_count = (stacks_raw.pop_front().unwrap().len() + 1) / 4;
    println!("Stack Count: {}", stack_count);
    let mut ret_val = Vec::<VecDeque<char>>::new();

    for _ in 0..stack_count { ret_val.push(VecDeque::<char>::new()) }

    
    let mut row = stacks_raw.pop_front();
    loop {
        let linestr: &str;
        match row {
            Some(thisstr) => linestr = thisstr,
            None => break
        }
        // j is the index into the string, i is the stack
        let mut j = 1;
        for i in 0..stack_count {
            let cur_crate = linestr.chars().nth(j).unwrap();
            if cur_crate != ' ' {
                ret_val[i].push_front(cur_crate);
            }
            j += 4;
        }
        row = stacks_raw.pop_front();
    }
    for i in 0..stack_count { print_stack(i, &ret_val[i]); }
    ret_val
}

fn print_stack(index: usize, stack: &VecDeque<char>)
{
    let mut output = format!("Stack {}: [", index);
    for c in stack {
        output += format!("{}, ", c).as_str();
    }
    output += "]";
    println!("{}", output);
}

fn parse_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(" ").collect();
    Move {
        num: parts[1].parse::<i32>().unwrap(),
        source: parts[3].parse::<i32>().unwrap() as usize,
        target: parts[5].parse::<i32>().unwrap() as usize
    }
}

fn perform_moves(moves: &Vec<Move>, stacks: &mut Vec<VecDeque<char>>) {
    for m in moves {
        for _ in 0..m.num {
            let val = stacks[m.source - 1].pop_front().unwrap();
            stacks[m.target - 1].push_front(val);
        }
    }
}