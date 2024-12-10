use std::{collections::HashSet, time::Instant};

fn run(data: &str) {
    let start = Instant::now();

    let map = data.as_bytes();
    let map_width = map
        .iter()
        .position(|&e| e == b'\n')
        .expect("Didn't find newline");

    //let map: Vec<u8> = map.iter().filter(|&&c| c != b'\n').map(|&c| c).collect();

    let mut stack = Vec::with_capacity(16000);

    for (i, ch) in map.iter().enumerate() {
        if *ch == b'0' {
            stack.push((i, *ch, i));
        }
    }

    let mut peak_by_source = HashSet::new();

    let mut total_trails = 0;

    //println!("{}", map[24 - map_width] as char);

    while let Some((i, height, source)) = stack.pop() {
        //println!("============");
        //println!("i {} height {} act {}", i, height, map[i] as char);
        //println!("stack {:?}", stack);
        if height == b'9' {
            peak_by_source.insert((i, source));
            total_trails += 1;
            continue;
        }
        // left
        if i > 0 && map[i - 1] == height + 1 {
            stack.push((i - 1, height + 1, source));
        }
        // up
        if i > map_width && map[i - map_width - 1] == height + 1 {
            stack.push((i - map_width - 1, height + 1, source));
        }
        // right
        if i < map.len() - 1 && map[i + 1] == height + 1 {
            stack.push((i + 1, height + 1, source));
        }
        // down
        if i < map.len() - map_width && map[i + map_width + 1] == height + 1 {
            stack.push((i + map_width + 1, height + 1, source));
        }
        //println!("stack {:?}", stack);
    }

    println!("Part 1: {}", peak_by_source.len());
    //println!("{:?}", peak_by_source);

    println!("Part 2: {}", total_trails);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let real = include_str!("../../in/10.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
