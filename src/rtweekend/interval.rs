pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }
    pub fn new_from(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    pub fn get_max(&self) -> f32 {
        self.max
    }
    pub fn get_min(&self) -> f32 {
        self.min
    }
    pub fn size(self) -> f32 {
        self.max - self.min
    }
    pub fn contains(&self, x: f32) -> bool {
        (self.min <= x) && (x <= self.max)
    }
    pub fn surrounds(&self, x: f32) -> bool {
        (self.min < x) && (x < self.max)
    }
}

const UNIVERSE: Interval = Interval {
    min: f32::NEG_INFINITY,
    max: f32::INFINITY,
};

const EMPTY: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};
