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

    let lined_data = data
        .lines()
        .dropping_back(1)
        .map(|l| l.as_bytes())
        .collect_vec();
    let line_length = lined_data[0].len();

    let mut ns = Vec::with_capacity(line_length);

    for i in 0..line_length {
        let mut n = 0;
        for line in &lined_data {
            let digit_at = line[i];
            if digit_at > b'0' && digit_at <= b'9' {
                n = (n * 10) + (digit_at - b'0') as u64;
            }
        }

        ns.push(n);
    }

    let groups = ns.split(|&n| n == 0).collect_vec();
    // println!("{:?}", groups);

    let p2: u64 = groups
        .iter()
        .zip(ops.iter())
        .map(|(&ns, &op)| {
            if op == "+" {
                ns.iter().sum::<u64>()
            } else {
                ns.iter().product()
            }
        })
        .sum();

    println!("Part 2: {}", p2);

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
