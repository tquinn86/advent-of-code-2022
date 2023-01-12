use std::fmt;
use std::fmt::Display;
use crate::position::Position;
use crate::grid::Grid;

impl Grid<Tile> {
  pub fn parse_line(&mut self, line: &str, locks: &mut Vec<Lock>) {
    //first split by colon, to get two halves and isolate the y coordinate
    //for the sensor
    let halves: Vec<&str> = line.split(":").collect();
    //then split by space
    let sensor_txt: Vec<&str> = halves[0].split(" ").collect();
    let beacon_txt: Vec<&str> = halves[1].split(" ").collect();
    //then split the coordinates by = to get the postions.
    let sensor_x = sensor_txt[2].split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
    let sensor_y = sensor_txt[3].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    let sensor_position = Position::new(sensor_x, sensor_y);

    println!("Sensor position: {:?}", sensor_position);

    let beacon_x = beacon_txt[5].split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
    let beacon_y = beacon_txt[6].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    let beacon_position = Position::new(beacon_x, beacon_y);
    println!("Beacon position: {:?}", beacon_position);

    let lock = Lock::new(sensor_position, beacon_position);
    locks.push(lock);

    self.block(beacon_position, Box::new(Tile::Beacon));
    self.block(sensor_position, Box::new(Tile::Sensor(lock)));
  }

  pub fn fill_blocks_on_line(&mut self, locks: &[Lock], line_number: i32) {
    // if the visible area (distance in a four directions) for a sensor, intersects
    // the line number fill in the blocked tiles on that line
    for l in locks {
      if ((l.sensor.y() <= line_number) && (l.sensor.y() + l.distance as i32) >= line_number)
         || ((l.sensor.y() >= line_number) && (l.sensor.y() - l.distance as i32) <= line_number) {
          println!("Found an intersecting lock: {:?} at distance {} from line {}", l, l.sensor.y().abs_diff(line_number), line_number);
          if !self.is_blocked(Position::new(l.sensor.x(), line_number)) {
            self.block(Position::new(l.sensor.x(), line_number), Box::new(Tile::Block));
          }
          for i in 1..(l.distance - l.sensor.y().abs_diff(line_number) + 1) {
            let pos_plus = Position::new(l.sensor.x() - i as i32, line_number);
            if !self.is_blocked(pos_plus) {
              self.block(pos_plus, Box::new(Tile::Block));
            }
            let pos_minus = Position::new(l.sensor.x() + i as i32, line_number);
            if !self.is_blocked(pos_minus) {
              self.block(pos_minus, Box::new(Tile::Block));
            }
          }
      }
    }
  }

  pub fn fill_blocks(&mut self) {
    let mut lock_blocks: Vec<Position> = vec![];
    for y in self.min_position().y()..=self.max_position().y() {
      for x in self.min_position().x()..=self.max_position().x() {
        let block = self.get(&Position::new(x, y));
        if let Some(t) = block {
          if let Tile::Sensor(l) = &**t {
            println!("Found Sensor at {:?}.", Position::new(x, y));
            self.get_lock_blocks(&l, &mut lock_blocks);
          } else {
            //println!("Found non sensor at {:?}", Position::new(x, y));
          }
        } else {
          //println!("Found nothing at {:?}", Position::new(x, y));
        }
      }
    }
    for b in lock_blocks {
      self.block(b, Box::new(Tile::Block));
    }
  }

  fn get_lock_blocks(&self, lock: &Lock, lock_blocks: &mut Vec<Position>) {
    for i in 0..=lock.distance {
      for j in 0..=(lock.distance - i) {
        let blocks: Vec<Position> = vec![Position::new(lock.sensor.x() - i as i32, lock.sensor.y() - j as i32), 
          Position::new(lock.sensor.x() + i as i32, lock.sensor.y() + j as i32), 
          Position::new(lock.sensor.x() + i as i32, lock.sensor.y() - j as i32), 
          Position::new(lock.sensor.x() - i as i32, lock.sensor.y() + j as i32) ];
        for b in blocks {
          if !self.is_blocked(b) {
            println!("Adding position {:?} to blocks.", b);
            lock_blocks.push(b);
          }
        }
      }
    }
  }

  pub fn get_blocks_on_line(&self, line: i32) -> u32 {
    let mut blocks = 0;
    for x in self.min_position().x()..=self.max_position().x() {
      let pos = Position::new(x, line);
      if self.is_blocked(pos) {
        let val = self.get(&pos).unwrap();
        if !val.is_beacon() {
          blocks += 1;
        }
      }
    }
    blocks
  }
}


#[derive(Debug)]
pub enum Tile {
  Beacon,
  Sensor(Lock),
  Block
}

#[derive(Debug, Clone, Copy)]
pub struct Lock {
  sensor: Position,
  beacon: Position,
  distance: u32 //always positive
}

impl Lock {
  pub fn new(sensor: Position, beacon: Position) -> Self {
    Self {
      beacon: beacon,
      sensor: sensor,
      distance: sensor.x().abs_diff(beacon.x()) + sensor.y().abs_diff(beacon.y())
    }
  }
}

impl Tile {
  pub fn is_beacon(&self) -> bool {
    match self {
      Tile::Beacon => true,
      _ => false
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tile::Beacon => write!(f, "B"),
      Tile::Sensor(_) => write!(f, "S"),
      Tile::Block => write!(f, "#")
    }
  }
}