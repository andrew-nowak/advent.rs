use advent_rs::must_parse;

#[derive(Debug)]
struct StackVal {
    so_far: u64,
    index: usize,
}

fn concat(l: u64, r: u64) -> u64 {
    let mut offset = 10;
    while r >= offset {
        offset *= 10;
    }
    l * offset + r
}

fn run(data: &str, may_concat: bool) {
    let mut total = 0;

    for line in data.lines() {
        let (goal, values) = line.split_once(": ").expect("Line was malformed");
        let values = values
            .split(" ")
            .map(|v| must_parse::<u64>(v))
            .collect::<Vec<u64>>();

        let goal = must_parse::<u64>(goal);

        let all_sum: u64 = values.iter().sum();
        let all_prod: u64 = values.iter().product();
        if all_sum == goal || all_prod == goal {
            total += goal;
            continue;
        }
        // no quick here answer here, need to examine it all :(
        let mut stack: Vec<StackVal> = Vec::with_capacity(2usize.pow(values.len() as u32));

        stack.push(StackVal {
            so_far: values[0],
            index: 0,
        });

        while let Some(pos) = stack.pop() {
            if pos.index + 1 == values.len() {
                // reached end of values, accumulator must equal the goal now or fail
                if pos.so_far == goal {
                    total += goal;
                    break;
                } else {
                    continue;
                }
            } else {
                let next_sum = pos.so_far + values[pos.index + 1];
                let next_prod = pos.so_far * values[pos.index + 1];
                // prevent states which cannot reach the goal
                if next_sum <= goal {
                    stack.push(StackVal {
                        so_far: next_sum,
                        index: pos.index + 1,
                    });
                }
                if next_prod <= goal {
                    stack.push(StackVal {
                        so_far: next_prod,
                        index: pos.index + 1,
                    });
                }
                if may_concat {
                    let next_concat = concat(pos.so_far, values[pos.index + 1]);
                    if next_concat <= goal {
                        stack.push(StackVal {
                            so_far: next_concat,
                            index: pos.index + 1,
                        });
                    }
                }
            }
        }
    }

    let part = if may_concat { 2 } else { 1 };

    println!("Part {}: {}", part, total);
}

fn main() {
    let example = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let real = include_str!("../../in/7.txt").trim();

    println!("--- example ---");
    run(example, false);
    run(example, true);

    println!("--- real ---");
    run(real, false);
    run(real, true);
}
