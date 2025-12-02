use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

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

fn find_sillys(s: &str) -> Vec<u64> {
    match s.split_once('-') {
        None => return vec![],
        Some((start, end)) => {
            let ustart = start.must_parse::<u64>();
            let uend = end.must_parse::<u64>();
            let half_digits = (if n_digits_b10(ustart) % 2 == 0 {
                n_digits_b10(ustart) / 2
            } else {
                n_digits_b10(uend) / 2
            }) as u32;
            let divisor = 10u64.pow(half_digits);
            ((ustart / divisor)..=(uend / divisor))
                .filter(|n| n_digits_b10(*n) == half_digits)
                .map(|n| n * divisor + n)
                .filter(|s| is_in_range(ustart, uend, *s))
                .dedup()
                .collect()
        }
    }
}

#[test]
fn test_find_sillys() {
    assert_eq!(find_sillys("11-22"), vec![11, 22]);
    assert_eq!(find_sillys("1188511880-1188511890"), vec![1188511885]);
    assert_eq!(find_sillys("1698522-1698528"), vec![]);
    assert_eq!(find_sillys("85-113"), vec![88, 99]);
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

    let sillys = ranges.flat_map(find_sillys);

    println!("Part 1: {}", sillys.sum::<u64>());

    let p2 = format!("hello {} v2", data);

    // println!("Part 2: {}", p2);

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
