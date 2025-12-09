use std::time::Instant;

use aoclib::{MustParse, Point};
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    location: Point,
    visited: Vec<Point>,
}

fn can_exit(target: &Point, obstacles: &FxHashSet<Point>) -> bool {
    let mut visited =
        FxHashSet::with_capacity_and_hasher((target.x * target.y) as usize, Default::default());

    let mut stack = Vec::new();

    stack.push(Point { x: 0, y: 0 });

    while let Some(point) = stack.pop() {
        if point == *target {
            return true;
        }
        for h in point.cardinal_neighbours().iter() {
            if h.within_zero_and(target) && !visited.contains(h) && !obstacles.contains(&h) {
                visited.insert(*h);
                stack.push(*h);
            }
        }
    }

    return false;
}

fn run(data: &str, width: i64, bytes: usize) {
    let start = Instant::now();

    let start_state = State {
        location: Point { x: 0, y: 0 },
        visited: Vec::new(),
    };
    let target = Point { x: width, y: width };

    let mut fallen_bytes = FxHashSet::default();

    for pos in data.lines().take(bytes) {
        let (xr, yr) = pos
            .split_once(",")
            .expect("pos line doesn't match expected");

        fallen_bytes.insert(Point {
            x: xr.must_parse::<i64>(),
            y: yr.must_parse::<i64>(),
        });
    }

    let mut bests = FxHashMap::default();
    bests.insert(Point { x: 0, y: 0 }, 0);

    let mut queue = Vec::new();
    queue.push(start_state);

    while let Some(pos) = queue.pop() {
        for h in pos.location.cardinal_neighbours().iter() {
            if h.within_zero_and(&target)
                && bests
                    .get(h)
                    .map(|&cx| pos.visited.len() + 1 < cx)
                    .unwrap_or(true)
                && !fallen_bytes.contains(&h)
            {
                bests.insert(*h, pos.visited.len() + 1);
                let mut next_state = State {
                    location: *h,
                    visited: pos.visited.clone(),
                };
                next_state.visited.push(*h);
                queue.push(next_state);
            }
        }
    }
    println!("part 1: {}", bests.get(&target).expect("Didn't finish?"));

    for extra_obst in data.lines().skip(bytes) {
        let (xr, yr) = extra_obst
            .split_once(",")
            .expect("pos line doesn't match expected");

        fallen_bytes.insert(Point {
            x: xr.must_parse::<i64>(),
            y: yr.must_parse::<i64>(),
        });

        if !can_exit(&target, &fallen_bytes) {
            println!("Part 2: {}", extra_obst);
            break;
        }
    }

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    println!("--- example ---");
    run(example, 6, 12);

    let real = include_str!("../../in/18.txt").trim();

    println!("--- real ---");
    run(real, 70, 1024);
}
