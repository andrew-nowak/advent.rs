use std::time::Instant;

use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

fn solve_p1(towels: &Vec<&str>, designs: &str) -> i32 {
    let mut possible = 0;

    'main: for design in designs.lines() {
        let mut stack = priority_queue::PriorityQueue::new();
        stack.push("".to_owned(), 0);

        let mut seen = FxHashSet::default();

        while let Some((pre, _len)) = stack.pop() {
            for towel in towels.iter() {
                let combo = pre.to_owned() + *towel;
                if seen.contains(&combo) {
                    continue;
                }
                if design == combo {
                    possible += 1;
                    continue 'main;
                }
                if design.starts_with(&combo) {
                    let len = combo.len();
                    seen.insert(combo.clone());
                    stack.push(combo, len);
                }
            }
        }
    }

    possible
}

fn solve_p2(towels: &Vec<&str>, designs: &str) -> (i32, i64) {
    let longest_towel = towels.iter().map(|t| t.len()).max().expect("no max??");
    let towels = FxHashSet::from_iter(towels.iter());

    let mut possible = 0;
    let mut combinations = 0;

    for design in designs.lines() {
        let mut stack = Vec::new();

        stack.push(design);

        let mut ways_to: FxHashMap<&str, i64> = FxHashMap::default();

        while let Some(remaining) = stack.last() {
            let mut all_found = true;
            let mut ways = 0;
            if towels.contains(&remaining) {
                ways += 1;
            }
            let remaining = *remaining;
            for l in 1..=longest_towel {
                if l >= remaining.len() {
                    continue;
                } else {
                    let back_pos = remaining.len() - l;
                    let (front, back) = remaining.split_at(back_pos);
                    if towels.contains(&back) {
                        match ways_to.get(front) {
                            Some(count) => {
                                ways += count;
                            }
                            None => {
                                all_found = false;
                                stack.push(front);
                            }
                        }
                    }
                }
            }

            if all_found {
                ways_to.insert(&remaining, ways);
                stack.pop();
            }
        }

        match ways_to.get(design) {
            Some(ways) if *ways > 0 => {
                possible += 1;
                combinations += ways;
            }
            _ => (),
        }
    }
    (possible, combinations)
}

fn run(data: &str) {
    let start = Instant::now();

    let (towels, designs) = data
        .split_once("\n\n")
        .expect("Can't split to towels and designs");

    let mut towels = towels.split(", ").collect::<Vec<&str>>();
    towels.sort_by_key(|t| t.len());

    //let p1 = solve_p1(&towels, designs);

    let (p1, p2) = solve_p2(&towels, designs);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    println!("--- example ---");
    run(example);

    let real = include_str!("../../in/19.txt").trim();

    println!("--- real ---");
    run(real);
}
