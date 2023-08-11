pub struct Seed {
    points: Vec<Point>,
}

impl Seed {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn glider_one(x: usize, y: usize) -> Self {
        Self::new(vec![
            Point::new(x, y),
            Point::new(x, y + 2),
            Point::new(x + 1, y + 1),
            Point::new(x + 1, y + 2),
            Point::new(x + 2, y + 1),
        ])
    }

    pub fn glider_two(x: usize, y: usize) -> Self {
        Self::new(vec![
            Point::new(x, y + 1),
            Point::new(x + 1, y + 2),
            Point::new(x + 2, y),
            Point::new(x + 2, y + 1),
            Point::new(x + 2, y + 2),
        ])
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
