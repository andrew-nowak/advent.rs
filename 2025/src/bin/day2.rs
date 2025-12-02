use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

// count the number of digits in the base10 rep of this number
fn n_digits_b10(i: u64) -> u32 {
    let mut i = i;
    let mut digits = 0;
    while i > 0 {
        i /= 10;
        digits += 1;
    }
    return digits;
}

fn is_in_range(start: u64, end: u64, i: u64) -> bool {
    i >= start && i <= end
}

type D2Range = (u64, u64);

// split a range into 2 ranges, each with equal number of digits (if necessary)
fn normalise_range_digits(r: D2Range) -> Vec<D2Range> {
    let (start, end) = r;

    let digits_start = n_digits_b10(start);
    let digits_end = n_digits_b10(end);

    if digits_start == digits_end {
        vec![r]
    } else {
        let boundary = 10u64.pow(digits_start);
        let start_range = (start, boundary - 1);
        let end_range = (boundary, end);
        vec![start_range, end_range]
    }
}

#[test]
fn test_normalise_range_digits() {
    assert_eq!(
        normalise_range_digits((95, 115)),
        vec![(95, 99), (100, 115)]
    );
    assert_eq!(
        normalise_range_digits((446443, 446449)),
        vec![(446443, 446449)]
    );
}

// after finding a valid prefix, build the matching invalid id
fn build_invalid(base: u64, repetition_times: u32, divisor: u64) -> u64 {
    let mut r = 0;
    for _ in 0..repetition_times {
        r = r * divisor + base;
    }
    r
}

#[test]
fn test_build_invalid() {
    assert_eq!(build_invalid(1, 3, 10), 111);
    assert_eq!(build_invalid(9, 2, 10), 99);
}

// find all invalid ids in this range which feature a repeating subsection of certain length
fn find_invalids_in_range_with_len(range: &D2Range, repetition_length: u32) -> Vec<u64> {
    let (start, end) = *range;
    let digits = n_digits_b10(start);
    if digits % repetition_length == 0 {
        let repetition_times = digits / repetition_length;
        let divisor = 10u64.pow(digits - repetition_length);
        // for all prefixes with given length in the range,
        ((start / divisor)..=(end / divisor))
            // build the invalid id that starts with that prefix
            .map(|base| build_invalid(base, repetition_times, 10u64.pow(repetition_length)))
            // then double check it's in the range
            .filter(|n| is_in_range(start, end, *n))
            .collect()
    } else {
        vec![]
    }
}

// find all invalid ids in this range
fn find_invalids_in_range(range: &D2Range) -> Vec<u64> {
    let (start, _) = *range;
    let digits = n_digits_b10(start);
    (1..=(digits / 2))
        .flat_map(|repetition_length| find_invalids_in_range_with_len(range, repetition_length))
        .collect()
}

#[test]
fn test_find_invalids_in_range() {
    assert_eq!(find_invalids_in_range(&(95, 99)), vec![99]);
    assert_eq!(
        find_invalids_in_range(&(9500, 9999)),
        vec![9999, 9595, 9696, 9797, 9898, 9999]
    );
}

// part 1: find all invalid ids with exactly two repetitions
fn find_invalids_of_two_reps(s: &str) -> Vec<u64> {
    match s
        .split_once('-')
        .map(|(start, end)| (start.must_parse::<u64>(), end.must_parse::<u64>()))
    {
        None => vec![],
        Some(range) => {
            let normalised_range = normalise_range_digits(range);
            normalised_range
                .iter()
                .flat_map(|range| {
                    let digits = n_digits_b10(range.0);
                    if digits % 2 == 0 {
                        find_invalids_in_range_with_len(range, digits / 2)
                    } else {
                        vec![]
                    }
                })
                .sorted()
                .dedup()
                .collect()
        }
    }
}

// part 2: find all invalids
fn find_invalids(s: &str) -> Vec<u64> {
    match s
        .split_once('-')
        .map(|(start, end)| (start.must_parse::<u64>(), end.must_parse::<u64>()))
    {
        None => vec![],
        Some(range) => {
            let normalised_range = normalise_range_digits(range);
            normalised_range
                .iter()
                .flat_map(find_invalids_in_range)
                .sorted()
                .dedup()
                .collect()
        }
    }
}

#[test]
fn test_find_invalids() {
    assert_eq!(find_invalids("11-22"), vec![11, 22]);
    assert_eq!(find_invalids("1188511880-1188511890"), vec![1188511885]);
    assert_eq!(find_invalids("1698522-1698528"), vec![]);
    assert_eq!(find_invalids("85-113"), vec![88, 99, 111]);
}

#[test]
fn test_b10_digit_counter() {
    assert_eq!(n_digits_b10(5), 1);
    assert_eq!(n_digits_b10(50), 2);
    assert_eq!(n_digits_b10(123456789), 9);
}

fn run(data: &str) {
    let start = Instant::now();

    let ranges = data.split(',');

    let invalids = ranges
        .clone()
        .flat_map(|range| find_invalids_of_two_reps(range));

    println!("Part 1: {}", invalids.sum::<u64>());

    let p2 = ranges.flat_map(|range| find_invalids(range));
    println!("Part 2: {}", p2.sum::<u64>());

    println!(
        "Extra: {}",
        find_invalids("1000000000-9999999999").iter().sum::<u64>()
    );

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let real = include_str!("../../in/2.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
