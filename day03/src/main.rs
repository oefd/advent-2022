mod rucksack;

use rucksack::Rucksack;

fn main() {
    let stdin = util::Stdin::new();
    let input: Vec<String> = stdin.lines().filter(|l| !l.is_empty()).collect();
    let rucksacks = input.iter().map(|line| Rucksack::from_str(line));
    let priority: u64 = rucksacks
        .map(|sack| priority(&sack.overlapping_item()))
        .sum();
    println!("rucksacks have sum of priorities: {}", priority);
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
