use std::time::Instant;

use rustc_hash::FxHashMap as HashMap;

use aoclib::MustParse;

fn run(data: &str) {
    let start = Instant::now();

    let initial_stones = data
        .split_whitespace()
        .map(|n| n.must_parse::<u64>())
        .collect::<Vec<u64>>();

    let mut stones = initial_stones.clone();

    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|&n| {
                let ns = n.to_string();
                if n == 0 {
                    vec![1]
                } else if ns.len() % 2 == 0 {
                    let (a, b) = ns.split_at(ns.len() / 2);
                    vec![a.must_parse::<u64>(), b.must_parse::<u64>()]
                } else {
                    vec![n * 2024]
                }
            })
            .collect::<Vec<u64>>();
    }

    println!("Part 1: {:?}", stones.len());

    let mut hs_stones: HashMap<u64, usize> = HashMap::default();

    for st in initial_stones {
        *hs_stones.entry(st).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut next_stones =
            HashMap::with_capacity_and_hasher(hs_stones.len() * 2, Default::default());
        for (&st, &count) in hs_stones.iter() {
            let ns = st.to_string();
            if st == 0 {
                next_stones.insert(1, count);
            } else if ns.len() % 2 == 0 {
                let (a, b) = ns.split_at(ns.len() / 2);
                *next_stones.entry(a.must_parse::<u64>()).or_insert(0) += count;
                *next_stones.entry(b.must_parse::<u64>()).or_insert(0) += count;
            } else {
                *next_stones.entry(st * 2024).or_insert(0) += count;
            }
        }
        hs_stones = next_stones;
    }

    let tot: usize = hs_stones.values().sum();

    println!("Part 2: {}", tot);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "125 17";

    let real = include_str!("../../in/11.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
