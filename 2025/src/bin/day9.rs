use std::time::Instant;

use aoclib::{MustParse, Point};
use itertools::Itertools;

fn area(a: &Point, b: &Point) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn within(a: i64, b: i64, c: i64) -> bool {
    let max = a.max(c);
    let min = a.min(c);

    min <= b && b <= max
}

fn strictly_within(a: i64, b: i64, c: i64) -> bool {
    let max = a.max(c);
    let min = a.min(c);

    min < b && b < max
}

fn within_shape(shape: &Vec<(Point, Point)>, p: &Point) -> bool {
    let mut lines_above = false;
    let mut lines_left = false;
    let mut lines_below = false;
    let mut lines_right = false;

    for line in shape.iter() {
        if line.0.x == p.x && line.1.x == p.x && within(line.0.y, p.y, line.1.y) {
            return true;
        } else if line.0.x <= p.x && line.1.x <= p.x && within(line.0.y, p.y, line.1.y) {
            lines_left = true;
        } else if line.0.x >= p.x && line.1.x >= p.x && within(line.0.y, p.y, line.1.y) {
            lines_right = true;
        } else if line.0.y == p.y && line.1.y == p.y && within(line.0.x, p.x, line.1.x) {
            return true;
        } else if line.0.y <= p.y && line.1.y <= p.y && within(line.0.x, p.x, line.1.x) {
            lines_above = true;
        } else if line.0.y >= p.y && line.1.y >= p.y && within(line.0.x, p.x, line.1.x) {
            lines_below = true;
        }
    }

    lines_above && lines_left && lines_below && lines_right
}

fn no_intersections(shape: &Vec<(Point, Point)>, a: &Point, b: &Point) -> bool {
    let other_corners = (Point { x: a.x, y: b.y }, Point { x: b.x, y: a.y });

    let lines = [
        (a, &other_corners.0),
        (&other_corners.0, b),
        (b, &other_corners.1),
        (&other_corners.1, a),
    ];

    for edge in shape.iter() {
        for line in lines.iter() {
            if line.0.x == line.1.x
                && edge.0.y == edge.1.y
                && strictly_within(edge.0.x, line.0.x, edge.1.x)
                && strictly_within(line.0.y, edge.0.y, line.1.y)
            {
                return false;
            }
            if line.0.y == line.1.y
                && edge.0.x == edge.1.x
                && strictly_within(edge.0.y, line.0.y, edge.1.y)
                && strictly_within(line.0.x, edge.0.x, line.1.x)
            {
                return false;
            }
        }
    }

    return true;
}

#[test]
fn test_wis() {
    let data = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let red_tiles = data
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").expect("Unexpected input line - no comma");
            let x = x.must_parse();
            let y = y.must_parse();
            Point { x, y }
        })
        .collect_vec();

    let mut shape: Vec<(Point, Point)> = red_tiles.iter().cloned().tuple_windows().collect_vec();
    shape.push((
        red_tiles.first().unwrap().clone(),
        red_tiles.last().unwrap().clone(),
    ));
    let shape = shape;

    assert_eq!(within_shape(&shape, &Point { x: 2, y: 3 }), true);
    assert_eq!(within_shape(&shape, &Point { x: 2, y: 5 }), true);
    assert_eq!(within_shape(&shape, &Point { x: 9, y: 5 }), true);
    assert_eq!(within_shape(&shape, &Point { x: 9, y: 3 }), true);
}

fn run(data: &str) {
    let start = Instant::now();

    let red_tiles = data
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").expect("Unexpected input line - no comma");
            let x = x.must_parse();
            let y = y.must_parse();
            Point { x, y }
        })
        .collect_vec();

    let pairs = red_tiles.iter().tuple_combinations();

    let pairs_by_area = pairs
        .map(|(a, b)| (a, b, area(a, b)))
        .sorted_by_key(|t| t.2)
        .rev()
        .collect_vec();

    let p1 = pairs_by_area.first().expect("Should be a first pair here");

    println!("Part 1: {}", p1.2);

    let mut shape: Vec<(Point, Point)> = red_tiles.iter().cloned().tuple_windows().collect_vec();
    shape.push((
        red_tiles.first().unwrap().clone(),
        red_tiles.last().unwrap().clone(),
    ));
    let shape = shape;

    let path = red_tiles
        .iter()
        .map(|t| format!("L {},{}", t.x, t.y))
        .collect::<String>();
    let first_tile = red_tiles.first().unwrap();
    let path = format!(
        "M {},{} {} L {},{}",
        first_tile.x, first_tile.y, path, first_tile.x, first_tile.y
    );

    let p2 = pairs_by_area
        .iter()
        .find(|(a, b, _area)| {
            let other_corners = (Point { x: a.x, y: b.y }, Point { x: b.x, y: a.y });
            within_shape(&shape, &other_corners.0)
                && within_shape(&shape, &other_corners.1)
                && no_intersections(&shape, a, b)
        })
        .expect("Soemthing should fit?");

    let a = p2.0;
    let b = p2.1;

    let other_corners = (Point { x: a.x, y: b.y }, Point { x: b.x, y: a.y });
    let selection = format!(
        "M {},{} L {},{} L {},{} L {},{} L {},{}",
        a.x,
        a.y,
        other_corners.0.x,
        other_corners.0.y,
        b.x,
        b.y,
        other_corners.1.x,
        other_corners.1.y,
        a.x,
        a.y
    );

    let svg = format!(
        r#"<svg viewBox="0 0 100000 100000" xmlns="http://www.w3.org/2000/svg">
            <path d="{}" fill="green" stroke="red" />
            <path d="{}" fill="red" />
        </svg>"#,
        path, selection,
    );

    std::fs::write("d9.svg", svg).unwrap();

    println!("Part 2: {}", p2.2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let real = include_str!("../../in/9.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
