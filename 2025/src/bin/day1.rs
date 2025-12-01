use std::time::Instant;

use aoclib::MustParse;

fn run(data: &str) {
    let start = Instant::now();

    let mut times_stopped_0 = 0;
    let mut times_past_0 = 0;
    let mut dial = 50;

    for line in data.lines() {
        if dial == 0 {
            times_stopped_0 += 1;
        }
        let (dir, dist) = line.split_at(1);
        let dist = dist.must_parse::<i32>();
        if dir == "R" {
            times_past_0 += dist / 100;
            let dist = dist % 100;
            if dial + dist > 100 {
                times_past_0 += 1;
            }
            dial += dist;
            dial %= 100;
        } else if dir == "L" {
            times_past_0 += dist / 100;
            let dist = dist % 100;
            if dial != 0 && dial - dist < 0 {
                times_past_0 += 1;
            }
            dial -= dist;
            dial += 100;
            dial %= 100;
        }
    }

    println!("Part 1: {}", times_stopped_0);

    println!("Part 2: {}", times_past_0 + times_stopped_0);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    let real = include_str!("../../in/1.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
