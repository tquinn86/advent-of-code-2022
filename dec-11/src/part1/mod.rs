use std::fmt;
use std::fmt::Display;
use std::collections::VecDeque;
use std::cell::RefCell;

#[derive(Copy, Clone)]
pub struct Item {
    worry_level: i32
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.worry_level)
    }
}

pub enum Operation {
    Add(i32),
    Mul(i32)
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
    test_divisor: i32,
    //target for true is test_targets.0
    //target for false is test_targets.1
    test_targets: (i32, i32),
    inspection_count: i32
}

impl Monkey {
    const SQUARED: i32 = -1;
    const BORED: i32 = 3;

    pub fn parse_monkey(monkey_lines: &[&str]) -> RefCell<Box<Self>> {
        let id = Self::parse_id(monkey_lines[0]);
        let mut items: Vec<i32> = vec![];
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
                        println!("    Worry level increases by itself to {}.", val + val);
                        val + val
                    } else {
                        println!("    Worry level increases by {} to {}.", a, val + a);
                        val + a
                    }
                },
                Operation::Mul(fac) => {
                    if fac == Monkey::SQUARED {
                        println!("    Worry level is multiplied by itself to {}.", val * val);
                        val * val
                    } else {
                        println!("    Worry level is multplied by {} to {}.", fac, val * fac);
                        val * fac
                    }
                }
            };

            //now divide this number by "bored"
            val /= Self::BORED;
            println!("    Monkey gets bored with item. Worry level is divided by {} to {}", Self::BORED, val);

            //now figure out which monkey to throw to.
            let target_monkey: i32;
            if val % self.test_divisor == 0 {
                println!("    Current worry level is divisible by {}", self.test_divisor);
                target_monkey = self.test_targets.0;
            } else {
                println!("    Current worry level is not divisible by {}", self.test_divisor);
                target_monkey = self.test_targets.1;
            }
            println!("    Item with worry level {} is thrown to monkey {}", val, target_monkey);
            ret.push((target_monkey as usize, Item { worry_level: val}));
        }
        ret
    }

    pub fn push_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    pub fn get_inspection_count(&self) -> i32 {
        self.inspection_count
    }

    fn new(id: i32, operation: Operation, test_divisor: i32, test_targets: (i32, i32), items: &[i32])  -> Self {
        Self {
            id,
            items: items.iter().map(|x| Item { worry_level: *x }).collect(),
            operation,
            test_divisor,
            test_targets,
            inspection_count: 0
        }
    }

    fn parse_id(line: &str) -> i32 {
        let int_slice = line.split(" ").collect::<Vec<&str>>()[1].split(":").collect::<Vec<&str>>()[0];
        let id = int_slice.parse::<i32>().unwrap();
        println!("Monkey {}:", id);
        id
    }

    fn parse_items(line: &str, items: &mut Vec<i32>) {
        let items_line = line.split(":").collect::<Vec<&str>>()[1];
        let parsed_items = items_line.split(",").collect::<Vec<&str>>().iter().map(|x| x.to_string().trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
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
        //so we use the constant SQUARED (-1)
        //to indicate that.
        let ops_val: i32;
        if parts[7] == "old" {
            ops_val = Self::SQUARED;
        } else {
            ops_val = parts[7].parse::<i32>().unwrap();
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

    fn parse_test_divisor(line: &str) -> i32 {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let divisor = parts[5].parse::<i32>().unwrap();
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