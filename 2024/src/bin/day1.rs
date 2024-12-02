use std::collections::HashMap;
use std::collections::VecDeque;

fn run(bs: &str) {
    let id_pairs = bs.split("\n").collect::<Vec<&str>>();
    let mut a1: VecDeque<i64> = VecDeque::with_capacity(id_pairs.len());
    let mut a2: VecDeque<i64> = VecDeque::with_capacity(id_pairs.len());

    for id_pair in id_pairs {
        let mut ids = id_pair.split_whitespace();
        a1.push_back(ids.next().unwrap().parse().unwrap());
        a2.push_back(ids.next().unwrap().parse().unwrap());
    }

    let mut a1 = Vec::from(a1);
    a1.sort();
    let mut a2 = Vec::from(a2);
    a2.sort();

    let mut tot = 0;

    for i in 0..a1.len() {
        tot += a1[i].abs_diff(a2[i]);
    }

    println!("Part 1: {}", tot);

    let mut a2_freq = HashMap::with_capacity(a2.len());
    for n in a2 {
        *a2_freq.entry(n).or_insert(0) += 1;
    }

    let mut p2_tot: i64 = 0;

    for m in a1 {
        p2_tot += m * a2_freq.get(&m).unwrap_or(&0);
    }

    println!("Part 2: {}", p2_tot);
}

fn main() {
    let example = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    let real = include_str!("../../in/1.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
