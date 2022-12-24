use crate::position::Position;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Blocker {
  Rock,
  Sand
}

#[derive(Debug)]
pub struct Grid {
  blocked_tiles: HashMap<Position, Blocker>,
  min_position: Position,
  max_position: Position
}

#[derive(Debug)]
pub struct SandUnit {
  position: Position
}

impl Grid {
  pub fn parse_grid(contents: &String) -> Self {

    let mut grid = Self {
      blocked_tiles: HashMap::new(),
      min_position: Position::default(),
      max_position: Position::default()
    };

    for line in contents.lines() {
      let positions: Vec<&str> = line.split(" -> ").collect();

      let mut i = 0;
      let mut position1 = Position::parse(positions[i]);
      loop {
        if i + 1 == positions.len() {
          break;
        }
        let position2 = Position::parse(positions[i + 1]);

        grid.fill(position1, position2);

        position1 = position2;
        i += 1
      }
    }
    grid
  }

  fn fill(&mut self, position1: Position, position2: Position) {
    //initialize min and max
    if self.min_position == Position::default() {
      self.min_position = Position::new(position1.x(), 0); //y min always 0
    }

    if self.max_position == Position::default() {
      self.max_position = position1;
    }

    println!("Filling from {:?} to {:?}", position1, position2);
    if position1.x() == position2.x() {
      let min_y = i32::min(position1.y(), position2.y());
      let max_y = i32::max(position1.y(), position2.y());
      if self.max_position.y() < max_y { self.max_position = Position::new(self.max_position.x(), max_y); }
      for i in min_y..=max_y {
        self.block(Position::new(position1.x(), i), Blocker::Rock);
      }
    } else if position1.y() == position2.y() {
      let min_x = i32::min(position1.x(), position2.x());
      let max_x = i32::max(position1.x(), position2.x());
      if self.min_position.x() > min_x { self.min_position = Position::new(min_x, self.min_position.y()); }
      if self.max_position.x() < max_x { self.max_position = Position::new(max_x, self.max_position.y()); }
      for i in min_x..=max_x {
        self.block(Position::new(i, position1.y()), Blocker::Rock);
      }
    } else {
      panic!("Invalid line positions: {:?} {:?}", position1, position2);
    }
  }

  pub fn block(&mut self, position: Position, blocker: Blocker) {
    self.blocked_tiles.insert(position, blocker);
  }

  pub fn max_y(&self) -> i32 {
    self.max_position.y()
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut res: fmt::Result = Ok(());
      for y in self.min_position.y()..=self.max_position.y() {
        for x in self.min_position.x()..=self.max_position.x() {
          let val = self.blocked_tiles.get(&Position::new(x, y));
          if let Some(b) = val {
            match b {
              Blocker::Sand => res = write!(f, "{}", "o"),
              Blocker::Rock => res = write!(f, "{}", "#"),
            }
          } else {
            res = write!(f, "{}", ".");
          }
          if res.is_err() { return res; }
        }
        res = writeln!(f);
        if res.is_err() { return res; }
      }
      res
    }
}

impl SandUnit {
  pub fn new() -> Self {
    Self {
      position: Position::new(500, 0) //starting point
    }
  }

  pub fn position(&self) -> Position {
    self.position
  }

  pub fn can_move(&self, grid: &Grid) -> bool {
    if grid.blocked_tiles.contains_key(&Position::new(self.position.x(), self.position.y() + 1)) &&
       grid.blocked_tiles.contains_key(&Position::new(self.position.x() - 1, self.position.y() + 1)) &&
       grid.blocked_tiles.contains_key(&Position::new(self.position.x() + 1, self.position.y() + 1)) {
      false
    } else {
      true
    }
  }

  pub fn do_move(&mut self, grid: &Grid) {
    if !self.can_move(grid) {
      //redundant check to safe:
      panic!("SandUnit cannot move. Call can_move before calling do_move!");
    }

    let down = Position::new(self.position.x(), self.position.y() + 1);
    if !grid.blocked_tiles.contains_key(&down) {
      self.position = down;
    } else {
      let left = Position::new(self.position.x() - 1, self.position.y() + 1);
      if !grid.blocked_tiles.contains_key(&left) {
        self.position = left
      } else {
        let right = Position::new(self.position.x() + 1, self.position.y() + 1);
        self.position = right;
      }
    }
  }
}
