#![feature(iter_array_chunks)]
mod monkey;

use monkey::Monkey;

fn main() {
    let stdin = util::Stdin::new();
    let mut lines = stdin.cleaned_lines().into_iter();
    let mut monkies: Vec<Monkey> = lines
        .by_ref()
        .array_chunks::<6>()
        .map(|monkey_def| Monkey::from(&monkey_def))
        .collect();

    for _round in 0..20 {
        round(&mut monkies);
    }

    let top_inspections =
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
            });
    let monkey_business = top_inspections.0 * top_inspections.1;

    println!("monkey business: {}", monkey_business);
}

fn round(monkies: &mut Vec<Monkey>) {
    for i in 0..monkies.len() {
        loop {
            if let Some(throw) = monkies[i].inspect_item() {
                monkies[throw.1].items.push_back(throw.0);
            } else {
                break;
            }
        }
    }
}
