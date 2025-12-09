use std::time::Instant;

use aoclib::{MustParse, Point};
use itertools::Itertools;

fn area(a: &Point, b: &Point) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
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

    let pairs_by_area = pairs.map(|(a, b)| area(a, b)).sorted().collect_vec();

    let p1 = pairs_by_area.last().expect("Should be a final pair here");

    println!("Part 1: {}", p1);

    // let p2 = format!("hello {} v2", data);
    //
    // println!("Part 2: {}", p2);

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
