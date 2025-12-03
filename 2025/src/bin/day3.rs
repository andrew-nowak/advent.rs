use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

#[test]
fn test_highest_joltage_for_bank() {
    assert_eq!(highest_joltage_for_bank("987654321111111", 2), 98);
    assert_eq!(highest_joltage_for_bank("811111111111119", 2), 89);
    assert_eq!(highest_joltage_for_bank("234234234234278", 2), 78);
    assert_eq!(highest_joltage_for_bank("818181911112111", 2), 92);
    assert_eq!(
        highest_joltage_for_bank("987654321111111", 12),
        987654321111
    );
    assert_eq!(
        highest_joltage_for_bank("811111111111119", 12),
        811111111119
    );
    assert_eq!(
        highest_joltage_for_bank("234234234234278", 12),
        434234234278
    );
    assert_eq!(
        highest_joltage_for_bank("818181911112111", 12),
        888911112111
    );
}

fn highest_joltage_for_bank(bank: &str, n_batteries: usize) -> u64 {
    let batteries = bank
        .split("")
        .filter(|n| !n.is_empty())
        .map(|n| n.must_parse::<u64>())
        .collect_vec();

    let mut highest_joltage_parts = vec![];
    let mut consumed_to_index = 0;

    for battery_index in 0..n_batteries {
        let remaining = n_batteries - battery_index;
        let window_end = batteries.len() - remaining + 1;
        let candidates = &batteries[consumed_to_index..window_end];
        let (chosen_i, choice) = candidates
            .iter()
            .enumerate()
            .rev() // need to reverse as max_by_key returns the _last_ max, not first
            .max_by_key(|k| k.1)
            .expect("must be a max");
        highest_joltage_parts.push(choice);
        consumed_to_index += chosen_i + 1;
    }

    highest_joltage_parts
        .iter()
        .fold(0, |acc, &&el| acc * 10 + el)
}

fn run(data: &str) {
    let start = Instant::now();

    let mut output_joltage_p1 = 0;

    for bank in data.lines() {
        output_joltage_p1 += highest_joltage_for_bank(bank, 2);
    }

    println!("Part 1: {}", output_joltage_p1);

    let mut output_joltage_p2 = 0;

    for bank in data.lines() {
        output_joltage_p2 += highest_joltage_for_bank(bank, 12);
    }

    println!("Part 2: {}", output_joltage_p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    let real = include_str!("../../in/3.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
