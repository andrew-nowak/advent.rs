use aoclib::{hmap, hset, MustParse};
use regex::Regex;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::{collections::VecDeque, time::Instant};

fn run(data: &str) {
    let start = Instant::now();

    let gate_re = Regex::new(r"(?<a>\S+) (?<op>AND|XOR|OR) (?<b>\S+) -> (?<o>\S+)")
        .expect("regex didn't compile??");

    let (preset_wires, gate_defs) = data
        .split_once("\n\n")
        .expect("no split between preset wires and gates??");

    let mut wires = hmap![];

    for p_wire in preset_wires.lines() {
        let (wire_name, active) = p_wire.split_once(": ").expect("wire couldn't be split??");
        wires.insert(wire_name, active == "1");
    }

    let mut gates = VecDeque::new();
    let mut connections: FxHashMap<&str, FxHashSet<&str>> = hmap![];

    for cap in gate_re.captures_iter(gate_defs) {
        match (cap.name("a"), cap.name("b"), cap.name("op"), cap.name("o")) {
            (Some(a), Some(b), Some(op), Some(o)) => {
                gates.push_back((a.as_str(), b.as_str(), op.as_str(), o.as_str()));
                connections
                    .entry(o.as_str())
                    .and_modify(|e| {
                        e.insert(a.as_str());
                        e.insert(b.as_str());
                    })
                    .or_insert(hset![a.as_str(), b.as_str()]);
            }
            _ => panic!("regex missed some group??"),
        }
    }

    while let Some((a, b, op, o)) = gates.pop_front() {
        match (wires.get(a), wires.get(b), wires.get(o)) {
            (_, _, Some(oo)) => panic!("{} already set to {}!", o, oo),
            (Some(a_val), Some(b_val), _) => {
                if op == "AND" {
                    wires.insert(o, *a_val && *b_val);
                } else if op == "OR" {
                    wires.insert(o, *a_val || *b_val);
                } else if op == "XOR" {
                    wires.insert(o, *a_val ^ *b_val);
                } else {
                    panic!("{} unknown op", op);
                }
            }
            _ => gates.push_back((a, b, op, o)),
        }
    }

    let mut p1 = 0i64;
    let mut x = 0i64;
    let mut y = 0i64;

    for (wire_name, value) in wires.into_iter() {
        if wire_name.starts_with("x") && value {
            let wire_no = wire_name.trim_start_matches('x').must_parse::<i32>();
            x |= 1 << wire_no;
        }
        if wire_name.starts_with("y") && value {
            let wire_no = wire_name.trim_start_matches('y').must_parse::<i32>();
            y |= 1 << wire_no;
        }
        if wire_name.starts_with("z") && value {
            let wire_no = wire_name.trim_start_matches('z').must_parse::<i32>();
            p1 |= 1 << wire_no;
        }
    }

    println!("Part 1: {}", p1);

    println!("Part 2:");
    println!("  x: {:#064b}", x);
    println!("  y: {:#064b}", y);
    println!("x+y: {:#064b}", x + y);
    println!("  z: {:#064b}", p1);

    for k in 0..64 {
        let z = format!("z{:#02}", k);
        trail(&connections, &z, 0);
    }

    //println!("Part 2: {}", p2);
    //no code for part 2: solved by hand

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn trail(haystack: &FxHashMap<&str, FxHashSet<&str>>, needle: &str, indent: usize) {
    if indent == 0 {
        println!("{}", needle);
    } else {
        println!("{}+ {}", " ".repeat(indent - 1), needle);
    }
    haystack
        .get(needle)
        .map(|ns| ns.iter().for_each(|n| trail(haystack, n, indent + 4)));
}

fn main() {
    let example = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    let real = include_str!("../../in/24.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
