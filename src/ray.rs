use crate::point::Point;

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
}