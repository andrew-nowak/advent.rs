use std::{collections::VecDeque, time::Instant};

use aoclib::MustParse;
use rustc_hash::FxHashMap;

fn evolve(x: i32) -> i32 {
    let x = ((x << 6) ^ x) & 16777215;
    let x = ((x >> 5) ^ x) & 16777215;
    let x = ((x << 11) ^ x) & 16777215;

    x
}

fn evolve2k(x: i32) -> i32 {
    let mut x = x;

    for _ in 0..2000 {
        x = evolve(x);
    }
    x
}

type Sequ = (i32, i32, i32, i32);

fn price(x: i32) -> i32 {
    x % 10
}

fn evolve2k_with_sale_prices(x: i32) -> FxHashMap<Sequ, i32> {
    let mut m = FxHashMap::default();

    let mut queue = VecDeque::new();
    queue.push_back(x);

    for _ in 0..2000 {
        if queue.len() >= 5 {
            queue.pop_front();
        }
        let x = evolve(*queue.back().expect("queue was empty?"));
        queue.push_back(x);
        if queue.len() < 5 {
            continue;
        }

        assert!(queue.len() == 5);

        let sequ: Sequ = (
            price(queue[1]) - price(queue[0]),
            price(queue[2]) - price(queue[1]),
            price(queue[3]) - price(queue[2]),
            price(queue[4]) - price(queue[3]),
        );

        m.entry(sequ).or_insert(price(queue[4]));
    }

    m
}

fn run(data: &str) {
    let start = Instant::now();

    let mut p1 = 0;
    for x in data.lines() {
        let x = x.must_parse::<i32>();
        let y = evolve2k(x);
        p1 += y as i64;
    }

    println!("Part 1: {}", p1);

    let mut m: FxHashMap<Sequ, i32> = FxHashMap::default();
    for x in data.lines() {
        let pricemap = evolve2k_with_sale_prices(x.must_parse::<i32>());
        for (&sequ, &price) in pricemap.iter() {
            m.entry(sequ).and_modify(|e| *e += price).or_insert(price);
        }
    }

    let p2 = m.values().max().expect("no max??");

    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
1
2
3
2024";

    let real = include_str!("../../in/22.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
