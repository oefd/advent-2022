#![feature(iter_array_chunks)]
use num::Integer;
mod monkey;

use monkey::Monkey;

fn main() {
    let stdin = util::Stdin::new();
    let mut lines = stdin.cleaned_lines().into_iter();
    let monkies: Vec<Monkey> = lines
        .by_ref()
        .array_chunks::<6>()
        .map(|monkey_def| Monkey::from(&monkey_def))
        .collect();
    part1(monkies.clone());
    part2(monkies);
}

fn part1(mut monkies: Vec<Monkey>) {
    for _round in 0..20 {
        round(&mut monkies, None);
    }

    let top = top_inspections(&monkies);
    let monkey_business = top.0 * top.1;
    println!("part 1 monkey business: {}", monkey_business);
}

fn part2(mut monkies: Vec<Monkey>) {
    let lcm = monkies
        .iter()
        .fold(1u64, |acc, monkey| Integer::lcm(&acc, &monkey.test_factor));
    for _round in 0..10_000 {
        round(&mut monkies, Some(lcm));
    }
    let top = top_inspections(&monkies);
    let monkey_business = top.0 * top.1;
    println!("part 2 monkey business: {}", monkey_business);
}

fn round(monkies: &mut Vec<Monkey>, mod_by: Option<u64>) {
    for i in 0..monkies.len() {
        loop {
            if let Some(throw) = monkies[i].inspect_item(mod_by) {
                monkies[throw.1].items.push_back(throw.0);
            } else {
                break;
            }
        }
    }
}

fn top_inspections(monkies: &Vec<Monkey>) -> (u64, u64) {
    monkies
        .iter()
        .map(|monkey| monkey.inspections)
        .fold((0u64, 0u64), |acc, inspections| {
            if inspections > acc.0 {
                (inspections, acc.0)
            } else if inspections > acc.1 {
                (acc.0, inspections)
            } else {
                acc
            }
        })
}
