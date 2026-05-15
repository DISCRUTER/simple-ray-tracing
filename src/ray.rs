use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }
    pub fn origin(self) -> Point {
        self.origin
    }
    pub fn direction(self) -> Point {
        self.direction
    }
    pub fn at(self, t: f32) -> Point {
        self.origin() + self.direction() * t
    } 
}

