use regex::Regex;

fn run(data: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut tot = 0;

    for (_, [a, b]) in re.captures_iter(data).map(|c| c.extract()) {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();

        tot += a * b;
    }

    println!("Part 1: {}", tot);

    let re2 =
        Regex::new(r"(?:(?<m>mul)\((?<a>\d+),(?<b>\d+)\)|(?<y>do)\(\)|(?<n>don't)\(\))").unwrap();

    let mut on = true;
    tot = 0;

    for capture in re2.captures_iter(data) {
        if capture.name("y").is_some() {
            on = true;
        } else if capture.name("n").is_some() {
            on = false;
        } else if on {
            let a = capture.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let b = capture.name("b").unwrap().as_str().parse::<i32>().unwrap();
            tot += a * b;
        }
    }

    println!("Part 2: {}", tot);
}

fn main() {
    let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let real = include_str!("../../in/3.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
