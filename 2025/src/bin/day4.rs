use std::time::Instant;

use aoclib::Point;
use rustc_hash::FxHashSet;

fn run(data: &str) {
    let start = Instant::now();

    let rolls: FxHashSet<Point> = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == '@' {
                    Some(Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    let accessible_rolls = rolls.iter().filter(|roll_loc| {
        roll_loc
            .all_neighbours()
            .iter()
            .filter(|roll_nbor| rolls.contains(&roll_nbor))
            .count()
            < 4
    });

    let p1 = accessible_rolls.count();

    println!("Part 1: {}", p1);

    let mut removed = 0;
    let mut rolls = rolls;

    loop {
        let removable: FxHashSet<Point> = rolls
            .iter()
            .cloned()
            .filter(|roll_loc| {
                roll_loc
                    .all_neighbours()
                    .iter()
                    .filter(|roll_nbor| rolls.contains(&roll_nbor))
                    .count()
                    < 4
            })
            .collect();
        if removable.is_empty() {
            break;
        }
        removed += removable.len();
        rolls = rolls.difference(&removable).cloned().collect();
    }

    println!("Part 2: {}", removed);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    let real = include_str!("../../in/4.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
