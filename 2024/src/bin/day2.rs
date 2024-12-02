fn run(data: &str) {
    let reports = data
        .trim()
        .split("\n")
        .map(|rep| {
            rep.split_whitespace()
                .map(|lvl| lvl.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<Vec<u16>>>();

    let mut safe = 0;
    let mut saveable = 0;

    for rep in reports {
        let increasing = rep[0] < rep[1];
        let rep_is_safe = rep.windows(2).all(|pair| {
            pair[0].abs_diff(pair[1]) <= 3
                && pair[0] != pair[1]
                && (pair[0] < pair[1]) == increasing
        });
        if rep_is_safe {
            safe += 1;
            continue;
        }
        let subreps = vec![rep.clone(); rep.len()];
        let mut subreps = subreps.iter().enumerate().map(|(k, subrep)| {
            subrep
                .iter()
                .enumerate()
                .filter_map(|(i, l)| if i == k { None } else { Some(*l) })
                .collect::<Vec<u16>>()
        });

        let rep_is_saveable = subreps.any(|subrep| {
            let increasing = subrep[0] < subrep[1];
            subrep.windows(2).all(|pair| {
                pair[0].abs_diff(pair[1]) <= 3
                    && pair[0] != pair[1]
                    && (pair[0] < pair[1]) == increasing
            })
        });

        if rep_is_saveable {
            saveable += 1;
        }
    }

    println!("Part 1: {}", safe);

    //let p2 = format!("hello {} v2", data);
    //
    println!("Part 2: {}", safe + saveable);
}

fn main() {
    let example = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    let real = include_str!("../../in/2.txt");

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
