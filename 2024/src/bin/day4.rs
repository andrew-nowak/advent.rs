fn run(data: &str) {
    let data = data.as_bytes();

    let ll = data
        .iter()
        .position(|&b| b == b'\n')
        .expect("no newlines in input")
        + 1;

    let mut found = 0;

    //fn is_xmas(data: &[u8], start: usize, dir: usize) -> bool {
    //    start.wrapping_add(3usize.wrapping_mul(dir)) >= 0
    //        && (start) + 3 * dir < data.len()
    //        && [
    //            data[start],
    //            data[start + dir],
    //            data[start + 2 * dir],
    //            data[start + 3 * dir],
    //        ] == *b"XMAS"
    //}

    for (i, b) in data.iter().enumerate() {
        if *b == b'X' {
            // right
            if i + 3 < data.len() && [data[i], data[i + 1], data[i + 2], data[i + 3]] == *b"XMAS" {
                found += 1;
            }
            // left
            if i >= 3 && [data[i], data[i - 1], data[i - 2], data[i - 3]] == *b"XMAS" {
                found += 1;
            }
            // down
            if i + 3 * ll < data.len()
                && [data[i], data[i + ll], data[i + 2 * ll], data[i + 3 * ll]] == *b"XMAS"
            {
                found += 1;
            }
            // up
            if i >= 3 * ll
                && [data[i], data[i - ll], data[i - 2 * ll], data[i - 3 * ll]] == *b"XMAS"
            {
                found += 1;
            }

            // down-right
            if i + 3 * ll + 3 < data.len()
                && [
                    data[i],
                    data[i + 1 + ll],
                    data[i + 2 + ll * 2],
                    data[i + 3 + ll * 3],
                ] == *b"XMAS"
            {
                found += 1;
            }
            // down-left
            if i - 3 + ll * 3 < data.len()
                && [
                    data[i],
                    data[i - 1 + ll],
                    data[i - 2 + ll * 2],
                    data[i - 3 + ll * 3],
                ] == *b"XMAS"
            {
                found += 1;
            }
            // up-left
            if i >= 3 + 3 * ll
                && [
                    data[i],
                    data[i - 1 - ll],
                    data[i - 2 - 2 * ll],
                    data[i - 3 - 3 * ll],
                ] == *b"XMAS"
            {
                found += 1;
            }
            // up-right
            if i >= (3 * ll) - 3
                && [
                    data[i],
                    data[i + 1 - ll],
                    data[i + 2 - 2 * ll],
                    data[i + 3 - 3 * ll],
                ] == *b"XMAS"
            {
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
