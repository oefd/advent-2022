use bigint::U512;

mod monkey;
mod operation;
mod test;

use monkey::Monkey;

fn main() {
    let stdin = util::Stdin::new();
    let lines = stdin.cleaned_lines();
    let mut monkies: Vec<Monkey> = parse_monkies(lines).collect();

    // for _ in 0..20 {
    //     perform_round(&mut monkies, false);
    // }

    for round in 0..1000 {
        if round % 1000 == 0 {
            println!("round {}...", round);
        }
        perform_round(&mut monkies, true);
    }
    for (i, monkey) in monkies.iter().enumerate() {
        println!("monkey {} inspected {}", i, monkey.inspections);
    }

    let most_active = monkies.iter().map(|m| &m.inspections).fold(
        (U512::from(0), U512::from(0)),
        |acc, inspections| {
            if inspections > &acc.0 {
                (inspections.clone(), acc.0)
            } else if inspections > &acc.1 {
                (acc.0, inspections.clone())
            } else {
                acc
            }
        },
    );
    let monkey_business = most_active.0 * most_active.1;
    println!("monkey business: {}", monkey_business);
}

fn perform_round(monkies: &mut Vec<Monkey>, round_2: bool) {
    for i in 0..monkies.len() {
        loop {
            if let Some((item, thrown_to)) = monkies[i].inspect(round_2) {
                monkies[thrown_to].items.push_back(item);
            } else {
                break;
            }
        }
    }
}

fn parse_monkies(iter: impl Iterator<Item = String>) -> impl Iterator<Item = Monkey> {
    let mut iter = iter.peekable();
    std::iter::from_fn(move || {
        if iter.peek().is_some() {
            let iter: &mut dyn Iterator<Item = String> = &mut iter;
            Some(Monkey::from(iter))
        } else {
            None
        }
    })
}
