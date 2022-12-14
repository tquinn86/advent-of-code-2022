use std::ops::Add;
use std::ops::AddAssign;

#[derive(Copy, Clone)]
pub struct Sprite {
    pos: i32
}

impl Sprite {
    pub fn new(pos: i32) -> Self {
        Self{
            pos
        }
    }

    pub fn draw(&self, pixel: i32) {
        if pixel == self.pos ||
           pixel == self.pos - 1 ||
           pixel == self.pos + 1 {
               print!("#");
           } else {
               print!(".");
           }
    }
}

impl Add<i32> for Sprite {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output
    {
        Self {
            pos: self.pos + other
        }
    }
}

impl AddAssign<i32> for Sprite {
 
    fn add_assign(&mut self, other: i32)
    {
        self.pos += other;
    }
}