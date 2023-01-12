#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Position {
    x: i32, 
    y: i32
}

impl Position {
    pub fn parse(coords: &str) -> Self {
        let coord_strs: Vec<&str> = coords.split(",").collect();
        Self {
            x: coord_strs[0].parse::<i32>().unwrap(),
            y: coord_strs[1].parse::<i32>().unwrap()
        }
    }

    pub fn new(x: i32, y: i32) -> Self{
        Self {
            x,
            y
        }
    }

    //read only accessors
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}