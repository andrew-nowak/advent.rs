use std::time::Instant;

use aoclib::{hmap, hset};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

fn iterate_classic_manifold(
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

fn upsert_timelines(next: &mut FxHashMap<usize, u64>, key: usize, timelines: &u64) {
    next.entry(key)
        .and_modify(|o| *o += timelines)
        .or_insert(*timelines);
}

fn iterate_quantum_manifold(
    active_tachyons: FxHashMap<usize, u64>,
    manifold_line: &&[u8],
) -> FxHashMap<usize, u64> {
    let mut next_tachyons =
        FxHashMap::with_capacity_and_hasher(active_tachyons.len() * 2, Default::default());

    for (&tachyon_pos, &timelines) in active_tachyons.iter() {
        if manifold_line[tachyon_pos] == b'^' {
            upsert_timelines(&mut next_tachyons, tachyon_pos - 1, &timelines);
            upsert_timelines(&mut next_tachyons, tachyon_pos + 1, &timelines);
        } else {
            upsert_timelines(&mut next_tachyons, tachyon_pos, &timelines);
        }
    }
    next_tachyons
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
        .fold((active_tachyons, 0), iterate_classic_manifold);

    println!("Part 1: {}", times_split);

    let active_tachyons = hmap!(start => 1);

    let timelines_by_final_pos = lines
        .iter()
        .step_by(2)
        .dropping(1)
        .fold(active_tachyons, iterate_quantum_manifold);

    let total_timelines: u64 = timelines_by_final_pos.values().sum();
    println!("Part 2: {}", total_timelines);

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
