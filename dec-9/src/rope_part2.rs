use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32, //I don't think this can go negative, but i32 just in case
    y: i32
}

pub struct Rope {
    knots: Vec<Position>,
    tail_history: HashSet<Position>
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position {{x:{} y:{}}}", self.x, self.y)
    }
}

impl Rope {
    pub fn new (size: usize) -> Self
    {
        let mut new_val = Self {
            knots: vec!(Position::default(); size),
            tail_history: HashSet::new()
        };
        new_val.tail_history.insert(Position::default());
        new_val
    }

    pub fn get_tail_history_count(&self) -> usize {
        self.tail_history.len()
    }

    pub fn move_head(&mut self, direction: Direction, steps: i32)
    {
        match direction {
            Direction::Right => self.move_right(steps),
            Direction::Left => self.move_left(steps),
            Direction::Up => self.move_up(steps),
            Direction::Down => self.move_down(steps)
        }
    }

    fn move_up(&mut self, steps: i32) {
        println!("Moving up {} step(s).", steps);
        for _ in 0..steps {
            // update the lead knot
            self.knots[0].y += 1;
            for i in 1..self.knots.len() {
                self.knots[i] = Self::move_next(self.knots[i - 1], self.knots[i]);
            }
            //record tail history
            println!("Tail is now at {}", self.knots[self.knots.len() - 1]);
            self.tail_history.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn move_down(&mut self, steps: i32) {
        println!("Moving down {} step(s).", steps);
        for _ in 0..steps {
            // update the lead knot
            self.knots[0].y -= 1;
            for i in 1..self.knots.len() {
                self.knots[i] = Self::move_next(self.knots[i - 1], self.knots[i]);
            }
            //record tail history
            println!("Tail is now at {}", self.knots[self.knots.len() - 1]);
            self.tail_history.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn move_right(&mut self, steps: i32) {
        println!("Moving right {} step(s).", steps);
        for _ in 0..steps {
            // update the lead knot
            self.knots[0].x += 1;
            for i in 1..self.knots.len() {
                self.knots[i] = Self::move_next(self.knots[i - 1], self.knots[i]);
            }
            //record tail history
            println!("Tail is now at {}", self.knots[self.knots.len() - 1]);
            self.tail_history.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn move_left(&mut self, steps: i32) {
        println!("Moving left {} step(s).", steps);
        for _ in 0..steps {
            // update the lead knot
            self.knots[0].x -= 1;
            for i in 1..self.knots.len() {
                self.knots[i] = Self::move_next(self.knots[i - 1], self.knots[i]);
            }
            //record tail history
            println!("Tail is now at {}", self.knots[self.knots.len() - 1]);
            self.tail_history.insert(self.knots[self.knots.len() - 1]);
        }
    }

    fn move_next(head: Position, cur: Position) -> Position {
        //following knots can move any direction
        let mut tail = cur;
        //each knot moves based on where the on ahead of it moved to
        //which, even if the head moved up, the later ones could get pulled
        //left or right of even down.
        if head.y - tail.y > 1 {
            tail.y += 1;

            //if we're moving tail
            //and it is not in the same column, pull it over one closer
            //column
            if tail.x > head.x {
                tail.x -= 1;
            } else if tail.x < head.x {
                tail.x += 1
            }
        } else if tail.y - head.y > 1 {
            tail.y -= 1;

            //if we're moving tail
            //and it is not in the same column, pull it over one closer
            //column
            if tail.x > head.x {
                tail.x -= 1;
            } else if tail.x < head.x {
                tail.x += 1
            }
        } else if head.x - tail.x > 1 {
            tail.x += 1;

            //if we're moving tail
            //and it is not in the same column, pull it over one closer
            //column
            if tail.y > head.y {
                tail.y -= 1;
            } else if tail.y < head.y {
                tail.y += 1
            }
        } else if tail.x - head.x > 1 {
            tail.x -= 1;

            //if we're moving tail
            //and it is not in the same row, pull it over one closer
            //row
            if tail.y > head.y {
                tail.y -= 1;
            } else if tail.y < head.y {
                tail.y += 1
            }
        }
        tail
    }
}

impl Direction {
    pub fn from_char(c: char) -> Direction {
        if c == 'U' { Direction::Up }
        else if c == 'D' { Direction::Down }
        else if c == 'L' { Direction::Left }
        else if c == 'R' { Direction::Right }
        else { panic!("Unrecognized Direction"); }
    }
}