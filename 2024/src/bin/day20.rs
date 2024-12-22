use aoclib::Point;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Instant;

fn run(data: &str) {
    let start = Instant::now();

    let mut walls = FxHashSet::default();

    let mut startpoint = None;
    let mut endpoint = None;
    let mut maxpoint = None;

    for (y, row) in data.lines().enumerate() {
        for (x, &ch) in row.as_bytes().iter().enumerate() {
            if ch == b'#' {
                walls.insert(Point::from(x, y));
            } else if ch == b'S' {
                startpoint = Some(Point::from(x, y));
            } else if ch == b'E' {
                endpoint = Some(Point::from(x, y));
            }
            maxpoint = Some(Point::from(x, y));
        }
    }

    let startpoint = startpoint.expect("no startpoint found??");
    let endpoint = endpoint.expect("no endpoint found??");
    let maxpoint = maxpoint.expect("no maxpoint found??");

    let mut position = startpoint;

    let mut visited = FxHashSet::default();
    let mut route = Vec::new();
    loop {
        if position == endpoint {
            break;
        }
        visited.insert(position);
        route.push(position);
        position = *position
            .cardinal_neighbours()
            .iter()
            .find(|p| p.within_zero_and(&maxpoint) && !walls.contains(p) && !visited.contains(p))
            .expect("didn't find next position??");
    }
    route.push(endpoint);

    //let route = FxHashMap::from_iter(route.iter().enumerate().map(|(i, &p)| (p, i)));
    //
    //let mut over_hundred_saved = 0;
    //
    //for (&pos, &step) in route.iter() {
    //    let cheat_destinations = [
    //        pos.up_by(2),
    //        pos.left_by(2),
    //        pos.down_by(2),
    //        pos.right_by(2),
    //    ];
    //
    //    for dest in cheat_destinations.iter() {
    //        if route
    //            .get(dest)
    //            .map(|&k| step + 100 + 2 <= k)
    //            .unwrap_or(false)
    //        {
    //            over_hundred_saved += 1;
    //        }
    //    }
    //}

    let mut p1 = 0;
    let mut p2 = 0;

    for i in 0..route.len() {
        for j in i..route.len() {
            let dist = route[i].manhattan(&route[j]);
            let i = i as i32;
            let j = j as i32;
            if dist <= 2 && (j - i - dist) >= 100 {
                p1 += 1;
            }
            if dist <= 20 && (j - i - dist) >= 100 {
                p2 += 1;
            }
        }
    }


    println!("Part 1: {}", p1);

    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    let real = include_str!("../../in/20.txt").trim();
    run(real);
}
