use std::time::Instant;

use aoclib::Point;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn fill(map: &HashMap<Point, u8>, start: &Point) -> HashSet<Point> {
    let plant = map.get(start).expect("start point wasn't in the map?");
    let mut stack: Vec<Point> = vec![*start];
    let mut region: HashSet<Point> = HashSet::default();

    while let Some(next) = stack.pop() {
        region.insert(next);
        let up = next.up();
        let down = next.down();
        let left = next.left();
        let right = next.right();
        if !region.contains(&up) && map.get(&up).is_some_and(|p| p == plant) {
            stack.push(up);
        }
        if !region.contains(&down) && map.get(&down).is_some_and(|p| p == plant) {
            stack.push(down);
        }
        if !region.contains(&left) && map.get(&left).is_some_and(|p| p == plant) {
            stack.push(left);
        }
        if !region.contains(&right) && map.get(&right).is_some_and(|p| p == plant) {
            stack.push(right);
        }
    }

    return region;
}

fn area_and_perimeter(region: &HashSet<Point>) -> (usize, usize) {
    let mut total = 0;
    for point in region.iter() {
        if !region.contains(&point.up()) {
            total += 1;
        }
        if !region.contains(&point.down()) {
            total += 1;
        }
        if !region.contains(&point.left()) {
            total += 1;
        }
        if !region.contains(&point.right()) {
            total += 1;
        }
    }
    (region.len(), total)
}

fn border(region: &HashSet<Point>) -> HashSet<Point> {
    region
        .iter()
        .filter(|&&p| !p.all_neighbours().iter().all(|nbor| region.contains(nbor)))
        .map(|p| p.clone())
        .collect()
}

fn corner(a: bool, b: bool, c: bool) -> bool {
    !a && !b || (a && b && !c)
}

fn run(data: &str) {
    let start = Instant::now();

    let width = data.bytes().position(|c| c == b'\n').expect("no newline");

    let mut map: HashMap<Point, u8> = HashMap::default();

    for (i, plant) in data.bytes().filter(|&c| c != b'\n').enumerate() {
        let point = Point {
            x: (i % width) as i64,
            y: (i / width) as i64,
        };
        assert!(!map.contains_key(&point));
        map.insert(point, plant);
    }

    let mut regions: Vec<(u8, HashSet<Point>)> = Vec::new();
    let mut region_assigned: HashSet<Point> = HashSet::default();

    for (point, plant) in map.iter() {
        if region_assigned.contains(&point) {
            continue;
        }
        let set = fill(&map, &point);

        region_assigned.extend(&set);
        regions.push((*plant, set));
    }

    let mut p1 = 0;

    for (_plant, region) in regions.iter() {
        let (area, perimeter) = area_and_perimeter(&region);
        p1 += area * perimeter;
    }

    println!("Part 1: {}", p1);

    let mut p2 = 0;

    for (_plant, region) in regions.iter() {
        let border = border(&region);
        let mut sides = 0;
        for p in border.iter() {
            if corner(
                region.contains(&p.up()),
                region.contains(&p.right()),
                region.contains(&p.up().right()),
            ) {
                sides += 1;
            }
            if corner(
                region.contains(&p.right()),
                region.contains(&p.down()),
                region.contains(&p.right().down()),
            ) {
                sides += 1;
            }
            if corner(
                region.contains(&p.down()),
                region.contains(&p.left()),
                region.contains(&p.down().left()),
            ) {
                sides += 1;
            }
            if corner(
                region.contains(&p.left()),
                region.contains(&p.up()),
                region.contains(&p.left().up()),
            ) {
                sides += 1;
            }
        }
        p2 += sides * region.len();
    }

    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
XXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXX
XXXXXAXXXXXXXXXX
XXXXXXXXXXXXXXXX
XXXXXXXXXXXXXXXX";

    let real = include_str!("../../in/12.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
