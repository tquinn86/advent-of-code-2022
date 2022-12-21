use std::cell::RefCell;
use std::collections::{HashMap, HashSet,VecDeque};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: Option<Position>,
    children: HashSet<Position>
}

impl Node {
    pub fn new(value: char) -> Self {
        if value == 'S' {
            Self {
                value: 1,
                parent: None,
                children: HashSet::new()
            }
        } else if value == 'E' {
            Self {
                value: 26,
                parent: None,
                children: HashSet::new()
            }
        } else {
            Self {
                value: (value as i32) - 96,
                parent: None,
                children: HashSet::new()
            }
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            parent: None,
            children: HashSet::new()
        }
    }
}

pub type NodeMap = HashMap<Position, RefCell<Box<Node>>>;

pub struct Grid {
    nodes: NodeMap,
    start_position: Position,
    end_position: Position,
    pub max_position: Position,
    invalid: bool
}

impl Grid {
    pub fn parse_grid(contents: &String) -> Grid {

        //We've got a big grid of letters.
        //the letters indicate relative height. a is lower than b, etc.
        //there is an S and an E somewhere in the grid
        //problem is to find the shortest path between S and E while never going 
        //up more than one (you can go to the same or down as much as you want)

        //I'm going to parse all of the letters into a HashMap using the
        //position (x,y) as a key. x counts up to the right, y counts up down

        let mut nodes: NodeMap = HashMap::new();

        let lines: Vec<&str> = contents.lines().collect();
        let max_y = lines.len() - 1;
        let max_x = lines[0].chars().collect::<Vec<char>>().len() - 1;
        let max_pos = Position { x: max_x as i32, y: max_y as i32 };

        let mut start_pos: Position = Position::default();
        let mut end_pos: Position = Position::default();

        for line in lines.iter().enumerate() {
            let y = line.0;

            for i in line.1.char_indices().enumerate() {

                let c = i.1.1;

                let pos = Position { x: i.0 as i32, y: y as i32 };

                let node = Node::new(c);

                // save off the start node
                if c == 'S' { start_pos = pos; }
                if c == 'E' { end_pos = pos; }

                nodes.insert(pos, RefCell::new(Box::new(node)));
            }
        }

        Grid::new(nodes, start_pos, end_pos, max_pos)
    }

    //part 2 is the same as part 1 except any "a" can be the starting position
    //so we'll need to clone the grid for each a, and re-build the tree.
    //each node is "clean" on clone (no parents or children)

    pub fn clone_with_new_start(&self, new_start: Position) -> Self {
        let mut nodes: NodeMap = HashMap::new();

        for i in 0..=self.max_position.y {
            for j in 0..=self.max_position.x {
                let p = Position{ x: j, y: i};
                nodes.insert(p, RefCell::new(Box::new(*self.nodes[&p].borrow().clone())));
            }
        }


        Self {
            nodes: nodes,
            start_position: new_start,
            end_position: self.end_position,
            max_position: self.max_position,
            invalid: false
        }
    }


    fn new(nodes: NodeMap, start_position: Position, end_position: Position, max_position: Position) -> Self {
        Self {
            nodes,
            start_position,
            end_position,
            max_position,
            invalid: false
        }
    }

    pub fn print_grid_chars(&self) {
        for i in 0..=self.max_position.y {
            for j in 0..=self.max_position.x {
                let p = Position{ x: j, y: i};
                let c: char;
                if p == self.start_position {
                    c = 'S';
                } else if p == self.end_position {
                    c = 'E';
                } else {
                    c = char::from_u32((self.nodes[&p].borrow().value as u32) + 96).unwrap();
                }

                print!("{}", c);
            }
            println!();
        }
    }

    fn is_valid(&self, p: Position) -> bool {

        let ret = p.x >= 0 && p.y >= 0 && p.x <= self.max_position.x && p.y <= self.max_position.y;

        //println!("Is_valid returning {:?} for position {:?}", ret, p);

        ret

    }

    pub fn get_value(&self, p: Position) -> i32 {
        self.nodes[&p].borrow().value
    }

    fn is_traversable(&self, p_from: Position, p_to: Position) -> bool {
        let from_val = self.get_value(p_from);
        let to_val = self.get_value(p_to);

        //true if to <= from, or only 1 more
        if to_val <= from_val || to_val == (from_val + 1) {
            //println!("Is traversable returning true for {:?} with value {:?} to {:?} with value {:?}", p_from, from_val, p_to, to_val);
            true
        } else {
            //println!("Is traversable returning false for {:?} with value {:?} to {:?} with value {:?}", p_from, from_val, p_to, to_val);
            false
        }
    }

    //part 2 is the same as part 1, except
    //any "a" can be a starting point

    pub fn build_tree(&mut self)
    {
        //queue of nodes to visit
        let mut q = VecDeque::<Position>::new();

        //set of nodes visited
        let mut v = HashSet::<Position>::new();

        //start both with the start node
        q.push_back(self.start_position);
        v.insert(self.start_position);

        loop {
            let cur_opt = q.pop_front();

            if cur_opt.is_none() {
                //can't get to E for start_position in this tree
                //so mark as invaild and quit
                self.invalid = true;
                break;
            }

            let cur = cur_opt.unwrap();
            //if cur is the end, we are done
            if cur == self.end_position {
                println!("Found the end, stopping.");
                break; 
            }

            //cur can have up to four children
            let directions: Vec<Position> = vec![Position{ x: cur.x, y: cur.y - 1 }, Position{ x: cur.x, y: cur.y + 1 }, Position{ x: cur.x - 1, y: cur.y }, Position{ x: cur.x + 1, y: cur.y }];

            for p in &directions {
                //if we haven't seen it before, it is a valid position (on the grid) 
                //and is traversable from cur (not too high) add it to the tree
                if !v.contains(p) && self.is_valid(*p) && self.is_traversable(cur, *p) {
                    //mark it as visited
                    v.insert(*p);
                    //add as a child to cur
                    self.add_child(cur, *p);
                    //set p's parent to cur, should only happen once per p
                    self.set_parent(*p, cur);
                    //and put p in the queue
                    q.push_back(*p);
                }
            }
        }
    }

    pub fn calc_shortest_path(&self) -> Result<i32, String> {
        if self.invalid {
            return Err("Invalid Tree".to_string());
        }
        
        let mut i = 0;
        let mut cur = self.nodes[&self.end_position].borrow();

        loop {
            if cur.parent.is_none() {
                return Ok(i); 
            } else {
                cur = self.nodes[&cur.parent.unwrap()].borrow();
            }
            i += 1;
        }
    }

    fn add_child(&self, parent: Position, child: Position) {
        let mut p = self.nodes[&parent].borrow_mut();

        p.children.insert(child);
    }

    fn set_parent(&self, child: Position, parent: Position) {
        //child can have only one parent
        //so panic if it is already set

        let mut c = self.nodes[&child].borrow_mut();
        if c.parent.is_some() { panic!("Trying to set parent to node {:?} more than once!", child); }

        c.parent = Some(parent);
    }
}
