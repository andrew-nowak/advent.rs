use aoclib::{hmap, MustParse};
use aoclib::{Direction, Point};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::Instant;

fn numeric_keypad() -> FxHashMap<char, Point> {
    let mut pad = FxHashMap::default();
    pad.insert('7', Point { x: 0, y: 0 });
    pad.insert('8', Point { x: 1, y: 0 });
    pad.insert('9', Point { x: 2, y: 0 });
    pad.insert('4', Point { x: 0, y: 1 });
    pad.insert('5', Point { x: 1, y: 1 });
    pad.insert('6', Point { x: 2, y: 1 });
    pad.insert('1', Point { x: 0, y: 2 });
    pad.insert('2', Point { x: 1, y: 2 });
    pad.insert('3', Point { x: 2, y: 2 });
    pad.insert('0', Point { x: 1, y: 3 });
    pad.insert('A', Point { x: 2, y: 3 });

    pad
}

fn direction_keypad() -> FxHashMap<char, Point> {
    let mut pad = FxHashMap::default();
    pad.insert('^', Point { x: 1, y: 0 });
    pad.insert('A', Point { x: 2, y: 0 });
    pad.insert('<', Point { x: 0, y: 1 });
    pad.insert('v', Point { x: 1, y: 1 });
    pad.insert('>', Point { x: 2, y: 1 });

    pad
}

fn cheapest_transitions() -> FxHashMap<Transition, FxHashMap<Transition, i64>> {
    let mut lookup = FxHashMap::default();

    lookup.insert(('A', 'A'), hmap! {('A', 'A') => 1});
    lookup.insert(('<', '<'), hmap! {('A', 'A') => 1});
    lookup.insert(('^', '^'), hmap! {('A', 'A') => 1});
    lookup.insert(('>', '>'), hmap! {('A', 'A') => 1});
    lookup.insert(('v', 'v'), hmap! {('A', 'A') => 1});

    lookup.insert(
        ('A', '<'),
        hmap! {
            ('A', 'v') => 1,
            ('v', '<') => 1,
            ('<', '<') => 1,
            ('<', 'A') => 1,
        },
    );
    lookup.insert(
        ('A', '^'),
        hmap! {
            ('A', '<') => 1,
            ('<', 'A') => 1,
        },
    );
    lookup.insert(
        ('A', '>'),
        hmap! {
            ('A', 'v') => 1,
            ('v', 'A') => 1,
        },
    );
    lookup.insert(
        ('A', 'v'),
        hmap! {
            ('A', '<') => 1,
            ('<', 'v') => 1,
            ('v', 'A') => 1,
        },
    );

    lookup.insert(
        ('<', 'A'),
        hmap! {
            ('A', '>') => 1,
            ('>', '>') => 1,
            ('>', '^') => 1,
            ('^', 'A') => 1,
        },
    );
    lookup.insert(
        ('<', '^'),
        hmap! {
            ('A', '>') => 1,
            ('>', '^') => 1,
            ('^', 'A') => 1,
        },
    );
    lookup.insert(
        ('<', '>'),
        hmap! {
            ('A', '>') => 1,
            ('>', '>') => 1,
            ('>', 'A') => 1,
        },
    );
    lookup.insert(
        ('<', 'v'),
        hmap! {
            ('A', '>') => 1,
            ('>', 'A') => 1,
        },
    );

    lookup.insert(
        ('^', 'A'),
        hmap! {
            ('A', '>') => 1,
            ('>', 'A') => 1,
        },
    );
    lookup.insert(
        ('^', '<'),
        hmap! {
            ('A', 'v') => 1,
            ('v', '<') => 1,
            ('<', 'A') => 1,
        },
    );
    lookup.insert(
        ('^', '>'),
        hmap! {
            ('A', 'v') => 1,
            ('v', '>') => 1,
            ('>', 'A') => 1,
        },
    );
    lookup.insert(
        ('^', 'v'),
        hmap! {
            ('A', 'v') => 1,
            ('v', 'A') => 1,
        },
    );

    lookup.insert(
        ('>', 'A'),
        hmap! {
            ('A', '^') => 1,
            ('^', 'A') => 1,
        },
    );
    lookup.insert(
        ('>', '^'),
        hmap! {
            ('A', '<') => 1,
            ('<', '^') => 1,
            ('^', 'A') => 1,
        },
    );
    lookup.insert(
        ('>', '<'),
        hmap! {
            ('A', '<') => 1,
            ('<', '<') => 1,
            ('<', 'A') => 1,
        },
    );
    lookup.insert(
        ('>', 'v'),
        hmap! {
            ('A', '<') => 1,
            ('<', 'A') => 1,
        },
    );

    lookup.insert(
        ('v', 'A'),
        hmap! {
            ('A', '^') => 1,
            ('^', '>') => 1,
            ('>', 'A') => 1,
        },
    );
    lookup.insert(
        ('v', '^'),
        hmap! {
            ('A', '^') => 1,
            ('^', 'A') => 1,
        },
    );
    lookup.insert(
        ('v', '>'),
        hmap! {
            ('A', '>') => 1,
            ('>', 'A') => 1,
        },
    );
    lookup.insert(
        ('v', '<'),
        hmap! {
            ('A', '<') => 1,
            ('<', 'A') => 1,
        },
    );

    lookup
}

fn dir_from_key(key: &char) -> Direction {
    match key {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("{} not a direction key", key),
    }
}

fn type_it(pad: &FxHashMap<char, Point>, cs: &Vec<char>) -> Vec<Vec<char>> {
    let mut arm_at = &'A';
    let mut presses: Vec<Vec<char>> = Vec::new();
    presses.push(Vec::new());

    let pad_points: Vec<Point> = Vec::from_iter(pad.values().copied());

    for c in cs.iter() {
        let mut section = Vec::new();
        let arm_position = pad
            .get(&arm_at)
            .expect(&format!("arm at {} which doesn't exist in pad", arm_at));
        let key_position = pad.get(&c).expect(&format!("{} doesn't exist in pad", c));

        let dx = key_position.x - arm_position.x;
        let dy = key_position.y - arm_position.y;

        if dx > 0 {
            for _ in 0..dx {
                section.push('>');
            }
        }
        if dy < 0 {
            for _ in 0..dy.abs() {
                section.push('^');
            }
        } else if dy > 0 {
            for _ in 0..dy {
                section.push('v');
            }
        }
        if dx < 0 {
            for _ in 0..dx.abs() {
                section.push('<');
            }
        }

        let section = section;

        let section_perms = section
            .iter()
            .copied()
            .permutations(section.len())
            .filter(|d| {
                let mut arm = *pad
                    .get(&arm_at)
                    .expect("arm not starting from pad location?");
                for mv in d.iter() {
                    arm = arm.go(&dir_from_key(mv));
                    if !pad_points.contains(&arm) {
                        return false;
                    }
                }
                true
            })
            .sorted()
            .dedup();
        let mut new_presses = Vec::new();
        for perm in section_perms {
            for press in presses.iter() {
                let mut new_press = press.clone();
                new_press.append(&mut (perm.clone()));
                new_press.push('A');

                new_presses.push(new_press);
            }
        }
        presses = new_presses;
        arm_at = c;
    }
    presses
}

fn select_cheapest_types(pad: &FxHashMap<char, Point>, seqs: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let all_seqs = seqs.iter().flat_map(|s| type_it(pad, s)).collect_vec();
    let cheapest_seq = all_seqs
        .iter()
        .map(|s| s.len())
        .min()
        .expect("no seq in seqs?");
    let all_of_cheapest_len = all_seqs
        .iter()
        .filter(|&s| s.len() == cheapest_seq)
        .cloned()
        .collect_vec();

    all_of_cheapest_len
    //let cheapest_with_most_runs = all_of_cheapest_len.iter().max_by_key(|s| {
    //    s.windows(2).map(|pair| if pair[0] == pair[1] { 4 } else { 0 }).sum::<i32>()
    //}).expect("bla");
    //vec!(cheapest_with_most_runs.clone())
}

type Transition = (char, char);

fn to_transitions(seqs: &Vec<Vec<char>>) -> Vec<FxHashMap<Transition, i64>> {
    seqs.iter()
        .map(|s| {
            let mut m = FxHashMap::default();
            let i = std::iter::once(&'A').chain(s);

            for (&a, &b) in i.tuple_windows() {
                m.entry((a, b)).and_modify(|e| *e += 1).or_insert(1);
            }
            m
        })
        .collect_vec()
}

fn extend_by_transitions(
    seqs: &Vec<FxHashMap<Transition, i64>>,
) -> Vec<FxHashMap<Transition, i64>> {
    let cheapest_transitions = cheapest_transitions();

    seqs.iter()
        .map(|seq| {
            let mut next_transitions = FxHashMap::default();

            for (transition, count) in seq.iter() {
                for (subsequent, subs_count) in cheapest_transitions
                    .get(transition)
                    .expect("transition missing from lookup?")
                {
                    let count = count * subs_count;
                    next_transitions
                        .entry(*subsequent)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
            }
            next_transitions
        })
        .collect_vec()
}

fn cheapest_route(routes: &Vec<FxHashMap<Transition, i64>>) -> i64 {
    let mut best = None;

    for route in routes.iter() {
        let tot = route.values().sum();
        match best {
            None => best = Some(tot),
            Some(prior_best) if tot < prior_best => best = Some(tot),
            _ => (),
        };
    }

    best.expect("no routes in routes?")
}

fn run(data: &str) {
    let start = Instant::now();

    let mut p1 = 0;
    let mut p2 = 0;

    for code in data.lines() {
        let code_chars = [code.chars().collect_vec()].to_vec();
        let bot_1 = select_cheapest_types(&numeric_keypad(), &code_chars);
        let bot_1_t = to_transitions(&bot_1);
        let bot_2 = extend_by_transitions(&bot_1_t);
        let bot_3 = extend_by_transitions(&bot_2);
        let bot_4 = extend_by_transitions(&bot_3);
        let bot_5 = extend_by_transitions(&bot_4);
        let bot_6 = extend_by_transitions(&bot_5);
        let bot_7 = extend_by_transitions(&bot_6);
        let bot_8 = extend_by_transitions(&bot_7);
        let bot_9 = extend_by_transitions(&bot_8);
        let bot_10 = extend_by_transitions(&bot_9);
        let bot_11 = extend_by_transitions(&bot_10);
        let bot_12 = extend_by_transitions(&bot_11);
        let bot_13 = extend_by_transitions(&bot_12);
        let bot_14 = extend_by_transitions(&bot_13);
        let bot_15 = extend_by_transitions(&bot_14);
        let bot_16 = extend_by_transitions(&bot_15);
        let bot_17 = extend_by_transitions(&bot_16);
        let bot_18 = extend_by_transitions(&bot_17);
        let bot_19 = extend_by_transitions(&bot_18);
        let bot_20 = extend_by_transitions(&bot_19);
        let bot_21 = extend_by_transitions(&bot_20);
        let bot_22 = extend_by_transitions(&bot_21);
        let bot_23 = extend_by_transitions(&bot_22);
        let bot_24 = extend_by_transitions(&bot_23);
        let bot_25 = extend_by_transitions(&bot_24);
        let bot_26 = extend_by_transitions(&bot_25);
        //println!("{}", bot_1.len());
        //let bot_2 = select_cheapest_types(&direction_keypad(), &bot_1);

        let code_num = (&code[..code.len() - 1]).must_parse::<i64>();

        p1 += code_num * cheapest_route(&bot_3) as i64;
        p2 += code_num * cheapest_route(&bot_26) as i64;
    }

    println!("Part 1: {}", p1);

    println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
029A
980A
179A
456A
379A";

    let real = include_str!("../../in/21.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
