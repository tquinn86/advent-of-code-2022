use std::fmt;
use std::fmt::Display;
use std::collections::VecDeque;
use std::cell::RefCell;

// NOTE: I went through a lot of tries
// it turns out the worry number, adjusted down to
// avoid overflow, must still be congruent across all
// possible targets. So the number must have the same
// modulo for each possible target to be valid
// there is a recursive function (find_congruent_n) below
// that goes through all of the moduli to find a number
// smaller than the calculated number that is congruent
// across all of the moduli. The worry number can stil get
// pretty large. Had change that (and all the components that operate on it)
// to u64

// See the README on the journey
// to get here.

#[derive(Copy, Clone)]
pub struct Item {
    worry_level: u64
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.worry_level)
    }
}


#[derive(Copy, Clone)]
pub enum Operation {
    Add(u64),
    Mul(u64)
}

impl Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add(a) => {
                if *a == Monkey::SQUARED {
                    write!(f, "  Operation: new = old + old")
                } else {
                    write!(f, "  Operation: new = old + {}", a)
                }
            },
            Operation::Mul(fac) => {
                if *fac == Monkey::SQUARED {
                    write!(f, "  Operation: new = old * old")
                } else {
                    write!(f, "  Operation: new = old * {}", fac)
                }
            }
        }
    }
}

pub struct Monkey {
    pub id: i32,
    //VecDeque so we can push_back and pop_front
    items: VecDeque<Item>,
    operation: Operation,
    test_divisor: u64,
    //target for true is test_targets.0
    //target for false is test_targets.1
    test_targets: (i32, i32),
    inspection_count: u64,
    moduli: Vec<u64>
}

impl Monkey {
    const SQUARED: u64 = u64::MAX;
 
    pub fn parse_monkey(monkey_lines: &[&str]) -> RefCell<Box<Self>> {
        let id = Self::parse_id(monkey_lines[0]);
        let mut items: Vec<u64> = vec![];
        Self::parse_items(monkey_lines[1], &mut items);
        let operation = Self::parse_operation(monkey_lines[2]);
        let test_divisor = Self::parse_test_divisor(monkey_lines[3]);
        let test_targets = Self::parse_test_targets(&monkey_lines[4..=5]);
        RefCell::new(Box::new(Self::new(id, operation, test_divisor, test_targets, &items)))
    }

    //this returns a vector of operations
    //the first number in the tuple is the
    //target monkey, the second is the Item to
    //throw to it

    pub fn operate(&mut self) -> Vec<(usize, Item)> {
        println!("Monkey {}:", self.id);

        let mut ret: Vec<(usize,Item)> = vec![];
        for _ in 0..self.items.len() {
            self.inspection_count += 1;
            let i = self.items.pop_front().unwrap();
            let mut val = i.worry_level;
            
            println!("  Monkey inspects item with worry level of {}.", val);
            //do the operation
            val = match self.operation {
                Operation::Add(a) => {
                    if a == Monkey::SQUARED {
                        let new_val = val + val;
                        println!("    Worry level increases by itself to {}.", new_val);
                        new_val
                    } else {
                        let new_val = val + a;
                        println!("    Worry level increases by {} to {}.", a, new_val);
                        new_val
                    }
                },
                Operation::Mul(fac) => {
                    if fac == Monkey::SQUARED {
                        let new_val = val * val;
                        println!("    Worry level is multiplied by itself to {}.", new_val);
                        new_val
                    } else {
                        let new_val = val * fac;
                        println!("    Worry level is multplied by {} to {}.", fac, new_val);
                        new_val
                    }
                }
            };

            //now figure out which monkey to throw to, and recalculate the value.
            let target_monkey: i32;
            if val % self.test_divisor == 0 {
                println!("    Current worry level is divisible by {}", self.test_divisor);
                target_monkey = self.test_targets.0;
            } else {
                println!("    Current worry level is not divisible by {} (mod is {})", self.test_divisor, val % self.test_divisor);
                target_monkey = self.test_targets.1;
            }
            println!("    Item with worry level {} is thrown to monkey {}", val, target_monkey);
            ret.push((target_monkey as usize, Item { worry_level: Self::find_congruent_n(val, &self.moduli) }));
        }
        ret
    }

    pub fn push_item(&mut self, item: Item) {
         self.items.push_back(item);
    }

    pub fn get_inspection_count(&self) -> u64 {
        self.inspection_count
    }

    pub fn get_modulus(&self) -> u64 {
        self.test_divisor
    }

    pub fn set_moduli(&mut self, moduli: &[u64])
    {
        for m in moduli {
            self.moduli.push(*m);
        }

        //sort biggest to smallest
        self.moduli.sort_by(|a, b| b.cmp(a));
    }

    fn new(id: i32, operation: Operation, test_divisor: u64, test_targets: (i32, i32), items: &[u64])  -> Self {
        Self {
            id,
            items: items.iter().map(|x| Item { worry_level: *x }).collect(),
            operation,
            test_divisor,
            test_targets,
            inspection_count: 0,
            moduli: vec![]
        }
    }

    fn parse_id(line: &str) -> i32 {
        let int_slice = line.split(" ").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0];
        let id = int_slice.parse::<i32>().unwrap();
        println!("Monkey {}:", id);
        id
    }

    fn parse_items(line: &str, items: &mut Vec<u64>) {
        let items_line = line.split(":").collect::<Vec<&str>>()[1];
        let parsed_items = items_line.split(",").collect::<Vec<&str>>().iter().map(|x| x.to_string().trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        
        print!("  Starting items:");
        for i in parsed_items {
            print!(" {}", i);
            items.push(i);
        }
        println!();
    }

    fn parse_operation(line: &str) -> Operation {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let ops_char = parts[6];

        //the ops_val can be either a number of the 
        //string "old" to indicate squaring (old * old)
        //or adding (old + old) the same value
        //so we use the constant SQUARED (u64::MAX)
        //to indicate that.
        let ops_val: u64;
        if parts[7] == "old" {
            ops_val = Self::SQUARED;
        } else {
            ops_val = parts[7].parse::<u64>().unwrap();
        }

        let op: Operation;
        if ops_char == "+" {
            op = Operation::Add(ops_val);
        } else if ops_char == "*" {
            op = Operation::Mul(ops_val);
        } else {
            panic!("Unrecognized ops character {}", ops_char);
        }
        
        println!("{}", op);
        op
    }

    fn parse_test_divisor(line: &str) -> u64 {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let divisor = parts[5].parse::<u64>().unwrap();
        println!("  Test: divisible by {}", divisor);
        divisor
    }

    fn parse_test_targets(lines: &[&str]) -> (i32, i32) {
        let true_val = lines[0].split(" ").collect::<Vec<&str>>()[9].parse::<i32>().unwrap();
        let false_val = lines[1].split(" ").collect::<Vec<&str>>()[9].parse::<i32>().unwrap();
        println!("    If true: throw to monkey {}", true_val);
        println!("    If false: throw to monkey {}", false_val);
        (true_val, false_val)
    }

    fn find_congruent_n(cur_val: u64, moduli: &[u64]) -> u64 {
        let mut mods_with_vals: Vec<(u64, u64)> = vec![];
        
        for m in moduli {
            //println!("{} mod {} is {}", cur_val, *m, cur_val % *m );
            mods_with_vals.push((*m, cur_val % *m));
        }

        let ret = Self::find_congruent_n_recursive(cur_val, mods_with_vals[0].1, mods_with_vals[0].0, &mods_with_vals);
        println!("Value adjusted to {}", ret);
        for m in moduli {
            //println!("{} mod {} is {}", ret, *m, ret % *m);
        }
        ret
    }

    fn find_congruent_n_recursive(cur_val: u64, candidate: u64, skip: u64, moduli: &[(u64, u64)]) -> u64 {
        if candidate >= cur_val || skip >= cur_val {
            println!("No candidate found, returning {}", cur_val);
            return cur_val;
        }

        if moduli.len() == 1 { 
            println!("Reached the bottom of the recursion, returning {}", candidate);
            return candidate;
        }

        //println!("Entering find_congruent_n_recursive. cur_val: {}, candidate: {}, skip: {}, moduli.len() == {}", cur_val, candidate, skip, moduli.len());

        let mut i = if candidate == 0 { skip } else { candidate };
        let mut new_candidate = 0;
        while i < cur_val {
            if i % moduli[1].0 == moduli[1].1 {
                //println!("found candidate {} ({} mod {} is {} and is supposed to be {})", i, i, moduli[1].0, i % moduli[1].0, moduli[1].1);
                new_candidate = i;
                break;
            }
            i = i + skip;
        }

        if new_candidate == 0 {
            println!("No candidate found, returning val {}", cur_val);
            return cur_val; 
        }

        return Self::find_congruent_n_recursive(cur_val, new_candidate, skip * moduli[1].0, &moduli[1..]);
    }


}

impl Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "Monkey {}:", self.id);
        if res.is_err() { return res; }
        for i in &self.items {
            res = write!(f, " {}", i);
            if res.is_err() { return res; }
        }
        res
    }
}