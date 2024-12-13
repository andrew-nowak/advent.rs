use aoclib::MustParse;
use std::time::Instant;

use regex::Regex;

fn run(data: &str) {
    let button_re =
        Regex::new(r"Button (?:A|B): X\+(\d+), Y\+(\d+)").expect("Failed compiling button_re");
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").expect("Failed compiling prize_re");

    let start = Instant::now();

    let mut tot = 0;
    let mut p2tot = 0;
    let p2_offset = 10000000000000;
    let machines = data.split("\n\n");

    for machine in machines {
        let mut lines = machine.lines();
        let aline = lines.next().expect("No A line for machine");
        let bline = lines.next().expect("No B line for machine");
        let prizeline = lines.next().expect("No prize line for machine");

        let aline = button_re.captures(aline).expect("A line didn't match");
        let ax = aline.get(1).unwrap().as_str().must_parse::<i64>();
        let ay = aline.get(2).unwrap().as_str().must_parse::<i64>();

        let bline = button_re.captures(bline).expect("B line didn't match");
        let bx = bline.get(1).unwrap().as_str().must_parse::<i64>();
        let by = bline.get(2).unwrap().as_str().must_parse::<i64>();

        let prizeline = prize_re
            .captures(prizeline)
            .expect("Prize line didn't match");
        let tx = prizeline.get(1).unwrap().as_str().must_parse::<i64>();
        let ty = prizeline.get(2).unwrap().as_str().must_parse::<i64>();

        let a = (bx * ty - by * tx) / (bx * ay - by * ax);
        let ar = (bx * ty - by * tx) % (bx * ay - by * ax);
        let b = (tx - (ax * a)) / bx;
        let br = (tx - (ax * a)) % bx;

        if ar == 0 && br == 0 {
            tot += 3 * a + b;
        }

        let tx = tx + p2_offset;
        let ty = ty + p2_offset;

        let a = (bx * ty - by * tx) / (bx * ay - by * ax);
        let ar = (bx * ty - by * tx) % (bx * ay - by * ax);
        let b = (tx - (ax * a)) / bx;
        let br = (tx - (ax * a)) % bx;

        if ar == 0 && br == 0 {
            p2tot += 3 * a + b;
        }

        //for apushes in 0..100 {
        //    for bpushes in 0..100 {
        //        if apushes * ax + bpushes * bx == tx && apushes * ay + bpushes * by == ty {
        //            tot += 3 * apushes + bpushes;
        //            continue 'machineloop;
        //        }
        //    }
        //}
    }

    println!("Part 1: {}", tot);

    println!("Part 2: {}", p2tot);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    let real = include_str!("../../in/13.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
