use aoclib::MustParse;
use std::time::Instant;

#[derive(Debug)]
enum Range {
    FileBlock(usize, usize),
    Space(usize),
}

fn run(data: &str) {
    let start = Instant::now();

    let sizes = data
        .trim()
        .split("")
        .filter(|c| !c.is_empty())
        .map(|c| c.must_parse::<usize>())
        .collect::<Vec<usize>>();

    let mut p1sizes = sizes.clone();

    let n = p1sizes.iter().sum();

    let mut harddrive = vec![0; n];

    let mut front_index = 0;
    let mut back_index = p1sizes.len() - 1;

    for i in 0..harddrive.len() {
        while p1sizes[front_index] == 0 {
            front_index += 1;
        }
        if back_index < front_index {
            break;
        }
        if front_index % 2 == 0 {
            harddrive[i] = front_index / 2;
            p1sizes[front_index] -= 1;
        } else {
            if p1sizes[back_index] == 0 {
                back_index -= 2;
            }
            harddrive[i] = back_index / 2;
            p1sizes[back_index] -= 1;
            p1sizes[front_index] -= 1;
        }
    }

    let mut checksum = 0;

    for (i, n) in harddrive.iter().enumerate() {
        checksum += i * n;
    }

    println!("Part 1: {}", checksum);

    /********** Part 2 **********/

    let p2sizes = sizes.clone();

    let mut p2sizes = p2sizes
        .iter()
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 0 {
                Range::FileBlock(i / 2, *n)
            } else {
                Range::Space(*n)
            }
        })
        .collect::<Vec<Range>>();

    let mut back_i = 0;
    p2sizes.reverse();

    while back_i < p2sizes.len() {
        if let Range::FileBlock(id, file_width) = p2sizes[back_i] {
            for d in (back_i..p2sizes.len()).rev() {
                if let Range::Space(space_width) = p2sizes[d] {
                    if file_width <= space_width {
                        p2sizes.splice(
                            d..d + 1,
                            [
                                Range::Space(space_width - file_width),
                                Range::FileBlock(id, file_width),
                            ],
                        );
                        p2sizes[back_i] = Range::Space(file_width);

                        break;
                    }
                }
            }
        }
        back_i += 1;
    }

    p2sizes.reverse();

    let mut harddrive = Vec::with_capacity(n);

    for range in p2sizes.iter() {
        match range {
            Range::Space(width) => {
                for _ in 0..*width {
                    harddrive.push(0);
                }
            }
            Range::FileBlock(id, width) => {
                for _ in 0..*width {
                    harddrive.push(*id);
                }
            }
        }
    }
    let mut checksum = 0;

    for (i, n) in harddrive.iter().enumerate() {
        checksum += i * *n;
    }

    println!("Part 2: {}", checksum);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "2333133121414131402";

    let real = include_str!("../../in/9.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
