#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn ccw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn within_zero_and(&self, other: &Point) -> bool {
        self.x >= 0 && self.x <= other.x && self.y >= 0 && self.y <= other.y
    }

    pub fn go(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Down => self.down(),
            Direction::Right => self.right(),
        }
    }

    pub fn up_by(&self, dist: i32) -> Point {
        Point {
            x: self.x,
            y: self.y - dist,
        }
    }
    pub fn up(&self) -> Point {
        self.up_by(1)
    }

    pub fn right_by(&self, dist: i32) -> Point {
        Point {
            x: self.x + dist,
            y: self.y,
        }
    }
    pub fn right(&self) -> Point {
        self.right_by(1)
    }
    pub fn down_by(&self, dist: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + dist,
        }
    }
    pub fn down(&self) -> Point {
        self.down_by(1)
    }
    pub fn left_by(&self, dist: i32) -> Point {
        Point {
            x: self.x - dist,
            y: self.y,
        }
    }
    pub fn left(&self) -> Point {
        self.left_by(1)
    }
}
