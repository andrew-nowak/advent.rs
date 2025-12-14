use std::{collections::VecDeque, time::Instant};

use aoclib::{hset, MustParse};
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashSet;

struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State {
    button_state: Vec<bool>,
    steps: usize,
}

fn solve(machine: &Machine) -> usize {
    let start = vec![false; machine.target.len()];

    let mut q = VecDeque::new();
    q.push_back(State {
        button_state: start.clone(),
        steps: 0,
    });

    let mut seen_states = hset![];

    while let Some(state) = q.pop_front() {
        if state.button_state == machine.target {
            return state.steps;
        }

        if seen_states.contains(&state) {
            continue;
        }
        seen_states.insert(state.clone());
        let next_states = machine
            .buttons
            .iter()
            .map(|button_press| {
                let mut next_state = State {
                    button_state: state.button_state.clone(),
                    steps: state.steps + 1,
                };
                for i in button_press.iter() {
                    next_state.button_state[*i] = !next_state.button_state[*i];
                }
                next_state
            })
            .filter(|s| !seen_states.contains(s));
        q.extend(next_states);
    }

    todo!()
}

fn build_machines(data: &str) -> Vec<Machine> {
    let indicators_re = Regex::new(r"\[([.#]+)\]").expect("Indicators regex did not compile");
    let buttons_re = Regex::new(r"\(([0-9,]+)\)").expect("buttons regex did not compile");

    let machines = data
        .lines()
        .map(|l| {
            let indicators = indicators_re
                .captures(l)
                .expect("No indicators match for line")
                .get(1)
                .expect("No group??")
                .as_str();
            let indicators = indicators
                .as_bytes()
                .iter()
                .map(|b| *b == b'#')
                .collect_vec();

            let buttons = buttons_re
                .captures_iter(l)
                .map(|c| c.extract::<1>().1[0])
                .map(|button_decl| {
                    button_decl
                        .split(",")
                        .map(|n| n.must_parse::<usize>())
                        .collect_vec()
                })
                .collect_vec();

            Machine {
                target: indicators,
                buttons,
            }
        })
        .collect_vec();

    machines
}

fn run(data: &str) {
    let start = Instant::now();

    let machines = build_machines(data);

    let p1 = machines.iter().map(solve).sum::<usize>();

    // let p1 = format!("hello {}", data);

    println!("Part 1: {}", p1);

    // let p2 = format!("hello {} v2", data);
    //
    // println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    let real = include_str!("../../in/10.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
