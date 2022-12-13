use std::collections::HashSet;

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32, //I don't think this can go negative, but i32 just in case
    y: i32
}

pub struct Rope {
    head: Position,
    tail: Position,
    tail_history: HashSet<Position>
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Rope {
    pub fn new () -> Self
    {
        let mut new_val = Self {
            head: Position::default(),
            tail: Position::default(),
            tail_history: HashSet::new()
        };
        new_val.tail_history.insert(Position::default());
        new_val
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

    pub fn get_tail_history_count(&self) -> usize {
        self.tail_history.len()
    }

    fn move_up(&mut self, steps: i32) {
        println!("Moving up {} step(s).", steps);
        for _i in 0..steps{
            //update head
            self.head.y += 1;

            //figure out where tail goes
            if self.head.y - self.tail.y > 1 {
                self.tail.y += 1;

                //if we're moving tail
                //and it is not in the same row, pull it into the same
                //row
                if self.tail.x != self.head.x {
                    self.tail.x = self.head.x;
                }
            }

            //update the history. No-op if it doesn't move.
            self.tail_history.insert(self.tail);
        }
    }

    fn move_down(&mut self, steps: i32) {
        println!("Moving down {} step(s).", steps);
        for _i in 0..steps{
            //update head
            self.head.y -= 1;

            //figure out where tail goes
            if self.tail.y - self.head.y > 1 {
                self.tail.y -= 1;

                //if we're moving tail
                //and it is not in the same row, pull it into the same
                //row
                if self.tail.x != self.head.x {
                    self.tail.x = self.head.x;
                }
            }

            //update the history. No-op if it doesn't move.
            self.tail_history.insert(self.tail);
        } 
    }

    fn move_right(&mut self, steps: i32) {
        println!("Moving right {} step(s).", steps);
        for _i in 0..steps{
            //update head
            self.head.x += 1;

            //figure out where tail goes
            if self.head.x - self.tail.x > 1 {
                self.tail.x += 1;

                //if we're moving tail
                //and it is not in the same column, pull it into the same
                //column
                if self.tail.y != self.head.y {
                    self.tail.y = self.head.y;
                }
            }

            //update the history. No-op if it doesn't move.
            self.tail_history.insert(self.tail);
        }
    }

    fn move_left(&mut self, steps: i32) {
        println!("Moving left {} step(s).", steps);
        for _i in 0..steps {
            //update head
            self.head.x -= 1;

            //figure out where tail goes
            if self.tail.x - self.head.x > 1 {
                self.tail.x -= 1;

                //if we're moving tail
                //and it is not in the same row, pull it into the same
                //row
                if self.tail.y != self.head.y {
                    self.tail.y = self.head.y;
                }
            }

            //update the history. No-op if it doesn't move.
            self.tail_history.insert(self.tail);
        } 
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