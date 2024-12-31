use std::time::Instant;

use itertools::Itertools;

type Key = Vec<usize>;
type Lock = Vec<usize>;

fn run(data: &str) {
    let start = Instant::now();

    let mut keys: Vec<Key> = vec![];
    let mut locks: Vec<Lock> = vec![];

    for item in data.split("\n\n") {
        let item = item.chars().collect_vec();
        let is_key = item[0] == '.';
        let mut heights = vec![];
        'cols: for col in 0..5 {
            for row in 1..6 {
                let offset = (row * 6) + col;
                if is_key && item[offset] == '#' {
                    heights.push(6 - row);
                    continue 'cols;
                } else if !is_key && item[offset] == '.' {
                    heights.push(row - 1);
                    continue 'cols;
                }
            }
            if is_key {
                heights.push(0);
            } else {
                heights.push(5);
            }
        }
        assert!(heights.len() == 5);
        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    let mut p1 = 0;

    for key in keys.iter() {
        'locks: for lock in locks.iter() {
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    continue 'locks;
                }
            }
            p1 += 1;
        }
    }

    println!("Part 1: {}", p1);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    let real = include_str!("../../in/25.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
