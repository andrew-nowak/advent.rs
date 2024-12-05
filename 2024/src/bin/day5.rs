use std::{cmp::Ordering, collections::HashMap};

fn run(data: &str) {
    let (rules, orders) = data.split_once("\n\n").unwrap();

    let mut rulemap: HashMap<&str, Vec<&str>> = HashMap::with_capacity(rules.len());

    for rule in rules.split('\n') {
        let (bef, aft) = rule.split_once('|').unwrap();
        rulemap
            .entry(aft)
            .and_modify(|bs| bs.push(bef))
            .or_insert(vec![bef]);
    }

    let mut tot = 0;

    let mut incorrect_orders = Vec::new();

    'outer: for order in orders.split('\n') {
        let order = order.split(',').collect::<Vec<&str>>();
        for i in 0..order.len() {
            for j in (i + 1)..order.len() {
                match rulemap.get(order[i]) {
                    Some(pages) if pages.contains(&order[j]) => {
                        incorrect_orders.push(order);
                        continue 'outer;
                    }
                    _ => (), // pass
                }
            }
        }
        tot += order[order.len() / 2].parse::<i32>().unwrap();
    }
    println!("Part 1: {:?}", tot);

    let mut tot = 0;

    let rules = rules
        .split('\n')
        .map(|r| r.split_once('|').unwrap())
        .collect::<Vec<(&str, &str)>>();

    for mut order in incorrect_orders {
        order.sort_by(|a, b| {
            if rules.contains(&(a, b)) {
                Ordering::Less
            } else if rules.contains(&(b, a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        tot += order[order.len() / 2].parse::<i32>().unwrap();
    }

    println!("Part 2: {}", tot);
}

fn main() {
    let example = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let real = include_str!("../../in/5.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
