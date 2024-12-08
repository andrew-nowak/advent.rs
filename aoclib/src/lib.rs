use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

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

pub trait MustParse {
    fn must_parse<F>(&self) -> F
    where
        F: FromStr,
        <F as FromStr>::Err: Debug;
}

impl MustParse for String {
    fn must_parse<F>(&self) -> F
    where
        F: FromStr,
        <F as FromStr>::Err: Debug,
    {
        self.parse::<F>().expect(&format!(
            "'{}' could not parse to {}",
            self,
            type_name::<F>()
        ))
    }
}

impl MustParse for &str {
    fn must_parse<F>(&self) -> F
    where
        F: FromStr,
        <F as FromStr>::Err: Debug,
    {
        self.parse::<F>().expect(&format!(
            "'{}' could not parse to {}",
            self,
            type_name::<F>()
        ))
    }
}

#[test]
fn test_must_parse_str() {
    assert_eq!("365".must_parse::<i32>(), 365);
    assert_eq!("-214".must_parse::<i32>(), -214);
}

#[test]
#[should_panic(expected = "'-214' could not parse to u32")]
fn test_must_parse_cannot_parse_neg_to_unsigned() {
    "-214".must_parse::<u32>();
}

#[test]
#[should_panic(expected = "'hi!' could not parse to i64")]
fn test_must_parse_failure_str() {
    "hi!".must_parse::<i64>();
}

#[test]
#[should_panic(expected = "'hi!' could not parse to usize")]
fn test_must_parse_failure_string() {
    String::from("hi!").must_parse::<usize>();
}
