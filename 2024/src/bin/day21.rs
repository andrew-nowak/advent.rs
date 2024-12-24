use aoclib::MustParse;
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
        .collect();
    all_of_cheapest_len
}

fn run(data: &str) {
    let start = Instant::now();

    let mut p1 = 0;

    for code in data.lines() {
        let code_chars = [code.chars().collect_vec()].to_vec();
        let bot_1 = select_cheapest_types(&numeric_keypad(), &code_chars);
        //println!("{}", bot_1.len());
        let bot_2 = select_cheapest_types(&direction_keypad(), &bot_1);
        let bot_3 = select_cheapest_types(&direction_keypad(), &bot_2);

        let code_num = (&code[..code.len() - 1]).must_parse::<i32>();

        p1 += code_num * bot_3[0].len() as i32;
    }

    println!("Part 1: {}", p1);

    //println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    //let example = "029A";
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
