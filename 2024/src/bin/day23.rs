use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Instant;

type Trio<'a> = (&'a str, &'a str, &'a str);

fn ordered_trio<'a>(a: &'a str, b: &'a str, c: &'a str) -> Trio<'a> {
    let mut ord = [a, b, c];
    ord.sort();
    (ord[0], ord[1], ord[2])
}

macro_rules! hset {
    ($($v:expr),* $(,)?) => {{
        FxHashSet::from_iter(std::iter::IntoIterator::into_iter([$($v,)*]))
    }};
}

fn run(data: &str) {
    let start = Instant::now();

    let mut connections: FxHashMap<&str, FxHashSet<&str>> =
        FxHashMap::with_capacity_and_hasher(1000, Default::default());

    for line in data.lines() {
        let (a, b) = line.split_once('-').expect("line was unsplittable??");
        connections
            .entry(a)
            .and_modify(|e| {
                e.insert(b);
                ()
            })
            .or_insert(hset![b]);
        connections
            .entry(b)
            .and_modify(|e| {
                e.insert(a);
                ()
            })
            .or_insert(hset![a]);
    }

    let mut trios: FxHashSet<Trio> = FxHashSet::default();
    let mut attempted_trios: FxHashSet<Trio> = FxHashSet::default();

    for (&puter, &ref connected) in connections.iter() {
        if !puter.starts_with("t") {
            continue;
        }

        for (a, b) in connected.into_iter().tuple_combinations() {
            let trio = ordered_trio(puter, a, b);
            if attempted_trios.contains(&trio) {
                continue;
            }
            attempted_trios.insert(trio);
            if connections
                .get(a)
                .expect("connection a wasn't in the map??")
                .contains(b)
            {
                trios.insert(trio);
            }
        }
    }

    println!("Part 1: {:?}", trios.len());

    'p2loop: for width in (1..14).rev() {
        for (&puter, &ref connected) in connections.iter() {
            let mut connected = connected.clone();
            connected.insert(puter);
            for combo in connected.into_iter().combinations(width) {
                let combo_set = FxHashSet::from_iter(combo.clone().into_iter());
                if combo.iter().all(|element| {
                    connections
                        .get(element)
                        .expect("element wasn't in connections??")
                        .intersection(&combo_set)
                        .count()
                        == width - 1
                }) {
                    let mut combo = combo;
                    combo.sort();
                    println!("Part 2 {}", combo.join(","));
                    break 'p2loop;
                }
            }
        }
    }

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    let real = include_str!("../../in/23.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
