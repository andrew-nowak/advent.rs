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
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn from(x: usize, y: usize) -> Point {
        Point {
            x: x as i64,
            y: y as i64,
        }
    }

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
        self.go_by(direction, 1)
    }

    pub fn go_by(&self, direction: &Direction, dist: i64) -> Point {
        match direction {
            Direction::Up => self.up_by(dist),
            Direction::Left => self.left_by(dist),
            Direction::Down => self.down_by(dist),
            Direction::Right => self.right_by(dist),
        }
    }

    pub fn up_by(&self, dist: i64) -> Point {
        Point {
            x: self.x,
            y: self.y - dist,
        }
    }
    pub fn up(&self) -> Point {
        self.up_by(1)
    }

    pub fn right_by(&self, dist: i64) -> Point {
        Point {
            x: self.x + dist,
            y: self.y,
        }
    }
    pub fn right(&self) -> Point {
        self.right_by(1)
    }
    pub fn down_by(&self, dist: i64) -> Point {
        Point {
            x: self.x,
            y: self.y + dist,
        }
    }
    pub fn down(&self) -> Point {
        self.down_by(1)
    }
    pub fn left_by(&self, dist: i64) -> Point {
        Point {
            x: self.x - dist,
            y: self.y,
        }
    }
    pub fn left(&self) -> Point {
        self.left_by(1)
    }

    pub fn cardinal_neighbours(&self) -> Vec<Point> {
        vec![self.up(), self.right(), self.down(), self.left()]
    }

    #[rustfmt::skip]
    pub fn all_neighbours(&self) -> Vec<Point> {
       vec![self.up().left()  , self.up()  , self.up().right(),
            self.left()       , /*        */ self.right(),
            self.down().left(), self.down(), self.down().right()]
    }

    pub fn manhattan(&self, o: &Point) -> i64 {
        (self.x.abs_diff(o.x) + self.y.abs_diff(o.y)) as i64
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    pub fn dist(&self, other: &Point3) -> f64 {
        let squared = (self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2);
        (squared as f64).sqrt()
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

#[macro_export]
macro_rules! hset {
    ($($v:expr),* $(,)?) => {{
        FxHashSet::from_iter(std::iter::IntoIterator::into_iter([$($v,)*]))
    }};
}

#[macro_export]
macro_rules! hmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        FxHashMap::from_iter(std::iter::IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}
