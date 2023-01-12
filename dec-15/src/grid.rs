use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use crate::position::Position;

#[derive(Debug)]
pub struct Grid<T> 
  where T: Display + Debug
{
  blocked_tiles: HashMap<Position, Box<T>>,
  min_position: Position,
  max_position: Position
}

impl<T: Display + Debug> Grid<T> {
  pub fn new() -> Self {
    Self {
      blocked_tiles: HashMap::new(),
      min_position: Position::new(i32::MAX, i32::MAX),
      max_position: Position::default()
    }
  }

  pub fn block(&mut self, position: Position, blocker: Box<T>) {
    println!("Blocking position {:?} with {:?}", position, *blocker);
    let res = self.blocked_tiles.insert(position, blocker);
    if let Some(v) = res {
      println!("Replacing {:?} at {:?}", v, position);
    }
    //grid can expand out to left and right
    if position.x() < self.min_position.x() {
      self.min_position = Position::new(position.x(), self.min_position.y());
    }

    if position.y() < self.min_position.y() {
      self.min_position = Position::new(self.min_position.x(), position.y());
    }

    if position.x() > self.max_position.x() {
      self.max_position = Position::new(position.x(), self.max_position.y());
    }

    if position.y() > self.max_position.y() {
      self.max_position = Position::new(self.max_position.x(), position.y());
    }
  }

  pub fn is_blocked(&self, position: Position) -> bool {
    self.blocked_tiles.contains_key(&position)
  }

  pub fn get(&self, position: &Position) -> Option<&Box<T>> {
    self.blocked_tiles.get(position)
  }

  pub fn max_position(&self) -> Position {
    self.max_position
  }

  pub fn min_position(&self) -> Position {
    self.min_position
  }
}

impl<T: Display + Debug> Display for Grid<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut res: fmt::Result = Ok(());
      for y in self.min_position.y()..=self.max_position.y() {
        for x in self.min_position.x()..=self.max_position.x() {
          let val = self.blocked_tiles.get(&Position::new(x, y));
          if let Some(b) = val {
            res = write!(f, "{}", b);
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