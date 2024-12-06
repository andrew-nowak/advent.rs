use std::collections::HashSet;

use advent_rs::{Direction, Point};

fn run_p2_iteration(
    map: &[&[u8]],
    bounds: &Point,
    start_pos: &Point,
    start_dir: &Direction,
) -> bool {
    let extra_obs = start_pos.go(&start_dir);

    let mut visited_w_dir: HashSet<(Point, Direction)> = HashSet::with_capacity(32000);
    let mut pos = start_pos.clone();
    let mut dir = start_dir.cw();

    loop {
        if visited_w_dir.contains(&(pos.clone(), dir.clone())) {
            return true;
        }
        visited_w_dir.insert((pos.clone(), dir.clone()));
        let next_pos = pos.go(&dir);
        if !next_pos.within_zero_and(bounds) {
            return false;
        }
        let next_tile = map[next_pos.y as usize][next_pos.x as usize];
        if next_tile == b'#' || next_pos.eq(&extra_obs) {
            dir = dir.cw();
        } else {
            pos = next_pos;
        }
    }
}

fn run(data: &str) {
    let data = data.as_bytes();

    let guard_start = data
        .iter()
        .position(|&b| b == b'^')
        .expect("Didn't find guard's starting position");

    let map: Vec<&[u8]> = data.split(|&b| b == b'\n').collect();

    let map_height = map.len();
    let map_width = map[0].len();

    let bounds = Point {
        x: map_width as i32 - 1,
        y: map_height as i32 - 1,
    };

    let start_pos = Point {
        x: (guard_start % (map_width + 1)) as i32,
        y: (guard_start / (map_width + 1)) as i32,
    };
    let start_dir = Direction::Up;

    let mut guard_pos = start_pos.clone();

    let mut guard_dir = start_dir;

    let mut visited: HashSet<Point> = HashSet::with_capacity(64000);

    let mut seen = 0;

    loop {
        visited.insert(guard_pos.clone());
        let next_pos = guard_pos.go(&guard_dir);
        if !next_pos.within_zero_and(&bounds) {
            break;
        }
        let next_tile = map[next_pos.y as usize][next_pos.x as usize];
        if next_tile == b'#' {
            guard_dir = guard_dir.cw();
        } else {
            if !visited.contains(&next_pos) 
                && run_p2_iteration(&map, &bounds, &guard_pos, &guard_dir)
            {
                seen += 1;
            }
            guard_pos = next_pos;
        }
    }

    println!("last pos: {:?}", guard_pos);

    println!("Part 1: {}", visited.len());

    //let p2 = format!("hello {} v2", data);
    //
    println!("Part 2: {}", seen); //loops.len());
    println!("bad {}", seen);
}

fn main() {
    let example = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    let real = include_str!("../../in/6.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
