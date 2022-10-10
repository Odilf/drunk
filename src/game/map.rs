use crate::game::Vector;

pub struct Map {
    height: i32
}

impl Map {
    pub fn new() -> Self {
        Map {
            height: 30,
        }
    }

    pub fn above_ground(&self, position: Vector) -> bool {
        position.y > self.height
    }
}
