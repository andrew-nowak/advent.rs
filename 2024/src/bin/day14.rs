use aoclib::{MustParse, Point};
use rustc_hash::FxHashSet as HashSet;
use std::thread;
use std::time::{Duration, Instant};

fn mod_to_positive(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Bot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn print_bots(bots: &HashSet<Point>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            if bots.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn run(data: &str, width: i32, height: i32) {
    let start = Instant::now();

    let robot_re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")
        .expect("failed to compile regex!");

    let t = 100;

    let mut ul = 0;
    let mut ur = 0;
    let mut bl = 0;
    let mut br = 0;

    let mut bots = Vec::new();

    for (_, [px, py, vx, vy]) in robot_re.captures_iter(data).map(|c| c.extract()) {
        let px = px.must_parse::<i32>();
        let py = py.must_parse::<i32>();
        let vx = vx.must_parse::<i32>();
        let vy = vy.must_parse::<i32>();

        bots.push(Bot { px, py, vx, vy });

        let px2 = mod_to_positive(px + t * vx, width);
        let py2 = mod_to_positive(py + t * vy, height);

        let left = px2 < width / 2;
        let right = px2 > width / 2;
        let upper = py2 < height / 2;
        let lower = py2 > height / 2;

        if upper && left {
            ul += 1;
        } else if upper && right {
            ur += 1;
        } else if lower && left {
            bl += 1;
        } else if lower && right {
            br += 1;
        }
    }

    println!("Part 1: {}", ul * ur * bl * br);

    for t in 0..100000 {
        let mut points = HashSet::default();

        for bot in bots.iter() {
            let x = mod_to_positive(bot.px + t * bot.vx, width);
            let y = mod_to_positive(bot.py + t * bot.vy, height);
            points.insert(Point { x, y });
        }

        if points
            .iter()
            .filter(|p| p.all_neighbours().iter().any(|n| points.contains(&n)))
            .count()
            > points.len() * 3 / 5
        {
            println!("                          ");
            println!("=========== {} ===========", t);
            print_bots(&points, width, height);

            thread::sleep(Duration::from_millis(750));
        }
    }

    //println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    let real = include_str!("../../in/14.txt").trim();

    println!("--- example ---");
    //run(example, 11, 7);

    println!("--- real ---");
    run(real, 101, 103);
}