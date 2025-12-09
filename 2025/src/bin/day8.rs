use std::time::Instant;

use aoclib::{hset, MustParse, Point3};
use itertools::Itertools;
use rustc_hash::FxHashSet;

fn run(data: &str, first_n: usize) {
    let start = Instant::now();

    let boxes = data
        .lines()
        .map(|l| {
            let (x, rem) = l.split_once(",").expect("First split failed");
            let (y, z) = rem.split_once(",").expect("Second split failed");

            let x: i64 = x.must_parse();
            let y: i64 = y.must_parse();
            let z: i64 = z.must_parse();

            Point3 { x, y, z }
        })
        .collect_vec();

    let pairs = boxes
        .iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .sorted_by(|(a1, a2), (b1, b2)| a1.dist(a2).total_cmp(&b1.dist(b2)))
        .collect_vec();

    let mut circuits: Vec<FxHashSet<Point3>> = boxes.iter().map(|&b| hset![b]).collect_vec();

    let mut conns_added = 0;

    for pair in pairs.iter() {
        if conns_added == first_n {
            let p1: u64 = circuits
                .iter()
                .sorted_by_key(|c| c.len())
                .rev()
                .take(3)
                .map(|c| c.len() as u64)
                .product();

            println!("Part 1: {}", p1);
        }

        let existing_left = circuits
            .iter()
            .position(|c| c.contains(pair.0))
            .expect("box should exist in a circuit");
        let existing_right = circuits
            .iter()
            .position(|c| c.contains(pair.1))
            .expect("box should exist in a circuit");

        if existing_left != existing_right {
            let rr = circuits[existing_right].clone();
            circuits[existing_left].extend(rr);
            circuits.remove(existing_right);
        }

        conns_added += 1;

        if circuits.len() == 1 {
            let p2 = pair.0.x * pair.1.x;

            println!("Part 2: {}", p2);
            break;
        }
    }

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    let real = include_str!("../../in/8.txt").trim();

    println!("--- example ---");
    run(example, 10);

    println!("--- real ---");
    run(real, 1000);
}
