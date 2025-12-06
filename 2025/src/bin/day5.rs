use std::time::Instant;

use aoclib::MustParse;
use itertools::Itertools;

#[allow(unused_variables)]
fn remove_overlaps_from_range(
    current_range: Option<(u64, u64)>,
    comparee: &(u64, u64),
) -> Option<(u64, u64)> {
    match (current_range, *comparee) {
        // range has already been eliminated, continue to end of fold
        (None, _) => None,
        // range doesn't overlap with comparee, so no change
        (Some((ca, cb)), (oa, ob)) if cb < oa => current_range,
        (Some((ca, cb)), (oa, ob)) if ob < ca => current_range,
        // range is entirely enclosed in comparee, so eliminate
        (Some((ca, cb)), (oa, ob)) if ca > oa && cb < ob => None,
        // the right-hand side of range overlaps with comparee
        (Some((ca, cb)), (oa, ob)) if ca < oa && cb <= ob => Some((ca, oa - 1)),
        // the left-hand side of range overlaps with comparee
        (Some((ca, cb)), (oa, ob)) if ca >= oa && cb > ob => Some((ob + 1, cb)),
        // if range is same as comparee, since that's a case
        (Some((ca, cb)), (oa, ob)) if ca == oa || cb == ob => None,

        (Some((ca, cb)), (oa, ob)) => {
            println!("{} {} {} {}", ca, cb, oa, ob);
            panic!("logic err!!! this case should be prevented by ordering")
        }
    }
}
fn add_range_without_overlaps(
    mut acc: Vec<(u64, u64)>,
    next_range: &(u64, u64),
) -> Vec<(u64, u64)> {
    let maybe_range_wo_overlaps = acc
        .iter()
        .fold(Some(*next_range), remove_overlaps_from_range);

    if let Some(range) = maybe_range_wo_overlaps {
        acc.push(range);
    }
    acc
}

fn run(data: &str) {
    let start = Instant::now();

    let (fresh_ranges, ids) = data
        .split_once("\n\n")
        .expect("Data missing ranges/ids separator");

    let fresh_ranges: Vec<(u64, u64)> = fresh_ranges
        .lines()
        .map(|range| {
            let (from, to) = range.split_once("-").expect("Range missing dash");

            (from.must_parse(), to.must_parse())
        })
        .collect_vec();

    let fresh = ids
        .lines()
        .map(|id| id.must_parse())
        .filter(|&id| {
            fresh_ranges
                .iter()
                .any(|&(from, end)| from <= id && id <= end)
        })
        .count();

    println!("Part 1: {}", fresh);

    let p2_ranges = fresh_ranges
        .iter()
        .sorted_by_key(|&(start, end)| end - start)
        .rev()
        .cloned()
        .collect_vec();

    let unoverlapped_ranges = p2_ranges.iter().fold(vec![], add_range_without_overlaps);

    let fresh_ids: u64 = unoverlapped_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    println!("Part 2: {}", fresh_ids);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

fn main() {
    let example = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    let real = include_str!("../../in/5.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
