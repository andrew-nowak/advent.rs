use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

fn run(data: &str) {
    let start = Instant::now();

    let (fresh_ranges, ids) = data
        .split_once("\n\n")
        .expect("Data missing ranges/ids separator");

    let fresh_ranges: Vec<(u64, u64)> = fresh_ranges
        .lines()
        .map(|range| {
            let (from, to) = range.split_once("-").expect("Range missing dash");

            (from.must_parse(), to.must_parse())
        })
        .collect_vec();

    let fresh = ids
        .lines()
        .map(|id| id.must_parse())
        .filter(|&id| {
            fresh_ranges
                .iter()
                .any(|&(from, end)| from <= id && id <= end)
        })
        .count();

    println!("Part 1: {}", fresh);

    // let p2 = format!("hello {} v2", data);

    // println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    let real = include_str!("../../in/5.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
