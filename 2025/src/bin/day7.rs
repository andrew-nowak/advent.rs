use std::time::Instant;

use aoclib::hset;
use itertools::Itertools;
use rustc_hash::FxHashSet;

fn iterate_splits(
    (active_tachyons, times_split): (FxHashSet<usize>, usize),
    manifold_line: &&[u8],
) -> (FxHashSet<usize>, usize) {
    let mut next_tachyons =
        FxHashSet::with_capacity_and_hasher(active_tachyons.len() * 2, Default::default());
    let mut splits = 0;
    for &tachyon in active_tachyons.iter() {
        if manifold_line[tachyon] == b'^' {
            splits += 1;
            next_tachyons.insert(tachyon - 1);
            next_tachyons.insert(tachyon + 1);
        } else {
            next_tachyons.insert(tachyon);
        }
    }

    (next_tachyons, times_split + splits)
}

fn run(data: &str) {
    let start_time = Instant::now();

    let lines = data.lines().map(|l| l.as_bytes()).collect_vec();

    let start = lines
        .first()
        .expect("No first line?")
        .iter()
        .position(|&b| b == b'S')
        .expect("First line has no start position?");

    let active_tachyons = hset!(start);

    let (_, times_split) = lines
        .iter()
        .step_by(2)
        .dropping(1)
        .fold((active_tachyons, 0), iterate_splits);

    println!("Part 1: {}", times_split);

    // let p2 = format!("hello {} v2", data);
    //
    // println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start_time).as_micros());
}

fn main() {
    let example = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    let real = include_str!("../../in/7.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
