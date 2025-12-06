use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

fn run(data: &str) {
    let start = Instant::now();

    let mut lines = data.lines().rev();

    let ops = lines
        .next()
        .expect("No first line??")
        .trim()
        .split_whitespace()
        .collect_vec();

    let numbers: Vec<Vec<u64>> = lines
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.must_parse::<u64>())
                .collect_vec()
        })
        .collect_vec();

    let total: u64 = ops
        .iter()
        .enumerate()
        .map(|(i, &op)| {
            if op != "+" && op != "*" {
                panic!("operator {} is unknown", op);
            }
            let sum = op == "+";
            let mut acc = if sum { 0 } else { 1 };

            for row in numbers.iter() {
                if sum {
                    acc += row[i];
                } else {
                    acc *= row[i];
                }
            }
            acc
        })
        .sum();

    println!("Part 1: {}", total);

    // let p2 = format!("hello {} v2", data);
    //
    // println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    let real = include_str!("../../in/6.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
