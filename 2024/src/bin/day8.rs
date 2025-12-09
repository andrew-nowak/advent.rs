use std::time::Instant;

use aoclib::Point;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn run(data: &str) {
    let start = Instant::now();
    let mut map: HashMap<u8, Vec<Point>> = HashMap::default();

    let bounds = Point {
        x: data.lines().next().expect("Couldn't find x bound").len() as i64 - 1,
        y: data.lines().count() as i64 - 1,
    };

    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.as_bytes().iter().enumerate() {
            if *ch != b'.' {
                let p = Point {
                    x: j as i64,
                    y: i as i64,
                };
                map.entry(*ch).or_default().push(p);
            }
        }
    }

    let map = map;

    let mut antinode_points: HashSet<Point> = HashSet::default();
    let mut resonant_antinode_points: HashSet<Point> = HashSet::default();

    for positions in map.values() {
        for p_a in positions.iter() {
            for p_b in positions.iter() {
                if *p_a == *p_b {
                    continue;
                }
                let diff = p_a.sub(p_b);
                let mut antinode = p_a.add(&diff);
                antinode_points.insert(antinode);
                resonant_antinode_points.insert(*p_a);
                resonant_antinode_points.insert(*p_b);
                resonant_antinode_points.insert(antinode);
                while antinode.within_zero_and(&bounds) {
                    antinode = antinode.add(&diff);
                    resonant_antinode_points.insert(antinode);
                }
            }
        }
    }

    let p1 = antinode_points
        .iter()
        .filter(|p| p.within_zero_and(&bounds))
        .count();

    println!("Part 1: {:?}", p1);

    let p2 = resonant_antinode_points
        .iter()
        .filter(|p| p.within_zero_and(&bounds))
        .count();

    println!("Part 2: {:?}", p2);

    println!("in {}μs", (Instant::now() - start).as_micros());
}

fn main() {
    let example = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let real = include_str!("../../in/8.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
