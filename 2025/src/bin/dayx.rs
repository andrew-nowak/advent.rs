use std::time::Instant;

fn run(data: &str) {
    let start = Instant::now();

    let p1 = format!("hello {}", data);

    println!("Part 1: {}", p1);

    let p2 = format!("hello {} v2", data);

    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "world";

    let real = "\
multi
line
data!"; // include_str!("../../in/X.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
