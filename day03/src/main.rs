#![feature(iter_array_chunks)]
use std::collections::HashSet;

mod rucksack;

use rucksack::Rucksack;

fn main() {
    let stdin = util::Stdin::new();
    let input: Vec<String> = stdin.lines().filter(|l| !l.is_empty()).collect();

    // part 1
    let rucksacks = input.iter().map(|line| Rucksack::from_str(line));
    let priority_sum: u64 = rucksacks
        .map(|sack| priority(&sack.overlapping_item()))
        .sum();
    println!("rucksacks have sum of priorities: {}", priority_sum);

    // part 2
    let groups = input.iter().array_chunks::<3>();
    let badges = groups.map(|group| {
        let rucksacks = &[
            Rucksack::from_str(group[0]),
            Rucksack::from_str(group[1]),
            Rucksack::from_str(group[2]),
        ];
        priority(&find_badge(&rucksacks))
    });
    let priority_sum: u64 = badges.sum();
    println!("groups have sum of badge priorities: {}", priority_sum);
}

fn priority(item: &char) -> u64 {
    let ascii_value: u64 = {
        let mut buf = [0u8; 1];
        item.encode_utf8(&mut buf);
        buf[0].into()
    };
    if item.is_ascii_uppercase() {
        ascii_value - 38
    } else if item.is_ascii_lowercase() {
        ascii_value - 96
    } else {
        panic!("non-ascii alphabetic character in compartment");
    }
}

fn find_badge(group: &[Rucksack; 3]) -> char {
    let sack_1_items = group[0].items();
    let sack_2_items = group[1].items();
    let intersection: HashSet<&char> = sack_1_items.intersection(&sack_2_items).collect();

    if let Some(badge) = group[2]
        .as_str()
        .chars()
        .find(|ch| intersection.contains(ch))
    {
        badge
    } else {
        panic!("expected shared item between all rucksacks in group");
    }
}
