use std::num::Wrapping;

fn run(data: &str) {
    let data = data.as_bytes();

    let ll = data
        .iter()
        .position(|&b| b == b'\n')
        .expect("no newlines in input")
        + 1;

    let mut found = 0;

    fn is_xmas(data: &[u8], start: Wrapping<usize>, dir: Wrapping<usize>) -> bool {
        // bounds check
        if (start + Wrapping(3) * dir).0 < data.len() {
            let run = [
                data[start.0],
                data[(start + dir).0],
                data[(start + Wrapping(2) * dir).0],
                data[(start + Wrapping(3) * dir).0],
            ];
            // forwards or backwards
            run == *b"XMAS" || run == *b"SAMX"
        } else {
            false
        }
    }

    for (i, b) in data.iter().enumerate() {
        if *b == b'X' || *b == b'S' {
            // left/right
            if is_xmas(data, Wrapping(i), Wrapping(1)) {
                found += 1;
            }
            // up/down
            if is_xmas(data, Wrapping(i), Wrapping(ll)) {
                found += 1;
            }

            // forward diag `/`
            if is_xmas(data, Wrapping(i), Wrapping(ll - 1)) {
                found += 1;
            }

            // backward diag `\`
            if is_xmas(data, Wrapping(i), Wrapping(ll + 1)) {
                found += 1;
            }
        }
    }

    println!("Part 1: {}", found);

    let mut found = 0;

    for i in 0..data.len() {
        if i < ll + 1 || i + ll + 1 >= data.len() || data[i] != b'A' {
            continue;
        }
        let x = [
            data[i - ll - 1],
            data[i - ll + 1],
            data[i + ll - 1],
            data[i + ll + 1],
        ];
        if x == *b"MSMS" || x == *b"MMSS" || x == *b"SMSM" || x == *b"SSMM" {
            found += 1;
        }
    }

    println!("Part 2: {}", found);
}

fn main() {
    let example = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
    .trim();

    let real = include_str!("../../in/4.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
