#![feature(array_chunks)]
use std::collections::HashSet;

fn main() {
    let stdin = util::Stdin::new();
    let input: Vec<String> = stdin.cleaned_lines().collect();
    let rucksacks: Vec<Rucksack> = input
        .iter()
        .map(|line| Rucksack::from_str(line.as_str()))
        .collect();

    // part 1
    let bad_sort_priorities: u64 = rucksacks
        .iter()
        .map(Rucksack::overlapping_item)
        .map(priority_from_char)
        .sum();
    println!("badly sorted items priority {}", bad_sort_priorities);

    // part 2
    let badge_priorities: u64 = rucksacks
        .array_chunks::<3>()
        .map(|group| find_group_badge(group))
        .map(priority_from_char)
        .sum();
    println!("badge items priority {}", badge_priorities);
}

fn find_group_badge(group: &[Rucksack]) -> char {
    let item_sets: Vec<HashSet<char>> = group.iter().map(|rs| rs.item_set()).collect();
    let intersection: HashSet<char> = item_sets
        .into_iter()
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .unwrap();
    assert!(intersection.len() == 1);
    intersection.into_iter().next().unwrap()
}

fn priority_from_char(ch: char) -> u64 {
    let ascii_value: u64 = {
        let mut buf = [0u8; 1];
        ch.encode_utf8(&mut buf);
        buf[0].into()
    };
    match ascii_value {
        0x41..=0x5A => ascii_value - 38,
        0x61..=0x7A => ascii_value - 96,
        _ => unreachable!(),
    }
}

struct Rucksack<'a> {
    definition: &'a str,
    compartments: [&'a str; 2],
}

impl<'a> Rucksack<'a> {
    pub fn from_str(s: &'a str) -> Self {
        let midpoint = s.len() / 2;
        Rucksack {
            definition: s,
            compartments: [&s[..midpoint], &s[midpoint..]],
        }
    }

    pub fn item_set(&self) -> HashSet<char> {
        HashSet::from_iter(self.definition.chars())
    }

    pub fn overlapping_item(&self) -> char {
        let left_items: HashSet<char> = HashSet::from_iter(self.compartments[0].chars());
        self.compartments[1]
            .chars()
            .find(|ch| left_items.contains(ch))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn badge_finding() {
        let group = [Rucksack::from_str("abcXYZ"), Rucksack::from_str("efgHIZ")];
        assert!(find_group_badge(&group) == 'Z');

        let group = [Rucksack::from_str("abcXYZ"), Rucksack::from_str("efgZIJ")];
        assert!(find_group_badge(&group) == 'Z');

        let group = [
            Rucksack::from_str("abcXYZ"),
            Rucksack::from_str("efgZIJ"),
            Rucksack::from_str("qZrwks"),
        ];
        assert!(find_group_badge(&group) == 'Z');

        let group = [
            Rucksack::from_str("abcXYZ"),
            Rucksack::from_str("nnZnnn"),
            Rucksack::from_str("Zeeeee"),
            Rucksack::from_str("tttZtt"),
        ];
        assert!(find_group_badge(&group) == 'Z');
    }

    #[test]
    #[should_panic]
    fn badge_finding_fails() {
        let group = [
            Rucksack::from_str("aaZaaa"),
            Rucksack::from_str("llllZl"),
            Rucksack::from_str("nnnnnn"),
        ];
        assert!(find_group_badge(&group) == 'Z');
    }
}
