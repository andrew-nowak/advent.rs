use std::cmp::Reverse;
use std::time::Instant;

use aoclib::Direction;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use aoclib::Point;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    score: i64,
    visited: Vec<Point>,
    location: Point,
    direction: Direction,
}

fn run(data: &str) {
    let start = Instant::now();

    let data = data.as_bytes();
    let area_width = data.iter().position(|&b| b == b'\n').expect("no newlines?") + 1;

    let startpoint = data.iter().position(|&b| b == b'S').expect("no start?");
    let startpoint = Point {
        x: (startpoint % area_width) as i64,
        y: (startpoint / area_width) as i64,
    };
    let endpoint = data.iter().position(|&b| b == b'E').expect("no end?");
    let endpoint = Point {
        x: (endpoint % area_width) as i64,
        y: (endpoint / area_width) as i64,
    };
    let walls = data
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| {
            if b == b'#' {
                Some(Point {
                    x: (i % area_width) as i64,
                    y: (i / area_width) as i64,
                })
            } else {
                None
            }
        })
        .collect::<HashSet<Point>>();

    let mut startpoint_vec = Vec::with_capacity(50);
    startpoint_vec.push(startpoint);

    let initial = State {
        score: 0,
        visited: startpoint_vec,
        location: startpoint,
        direction: Direction::Right,
    };

    let mut pq = PriorityQueue::with_capacity(100_000);
    pq.push(initial.clone(), Reverse(0));

    let mut best = i64::MAX;
    let mut cheapest_to_state = HashMap::default();
    cheapest_to_state.insert((startpoint, Direction::Right), 0);

    let mut best_paths = HashSet::default();

    'mainloop: while let Some((state, _)) = pq.pop() {
        if state.score > best {
            break;
        }
        let mut turn_cw = state.clone();
        turn_cw.direction = turn_cw.direction.cw();
        turn_cw.score += 1000;
        let mut turn_ccw = state.clone();
        turn_ccw.direction = turn_ccw.direction.ccw();
        turn_ccw.score += 1000;
        let ahead = state.location.go(&state.direction);
        let forward = if walls.contains(&ahead) {
            None
        } else {
            let mut s = state.clone();
            s.location = ahead;
            s.visited.push(ahead);
            s.score += 1;
            Some(s)
        };

        for next_state in [Some(turn_cw), Some(turn_ccw), forward] {
            match next_state {
                Some(next_state) if next_state.location == endpoint && next_state.score < best => {
                    println!("Part 1: {}", next_state.score);
                    // break 'mainloop;
                    best = next_state.score;
                    best_paths.extend(next_state.visited);
                }
                Some(next_state) if next_state.location == endpoint && next_state.score == best => {
                    best_paths.extend(next_state.visited);
                }
                Some(next_state) if next_state.score > best => (),
                Some(next_state)
                    if cheapest_to_state
                        .get(&(next_state.location, next_state.direction))
                        .map(|&cost| cost < next_state.score)
                        .unwrap_or(false) =>
                {
                    ()
                }
                Some(next_state) => {
                    cheapest_to_state.insert(
                        (next_state.location, next_state.direction),
                        next_state.score,
                    );
                    pq.push(next_state.clone(), Reverse(next_state.score));
                }
                None => (),
            }
        }
    }

    println!("Part 2: {}", best_paths.len());

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    let real = include_str!("../../in/16.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
