use std::collections::HashSet;

pub struct Rucksack<'a> {
    compartments_def: &'a str,
    pub compartments: [Compartment<'a>; 2],
}

impl<'a> Rucksack<'a> {
    pub fn from_str(s: &'a str) -> Self {
        assert!(s.len() % 2 == 0);

        let midpoint = s.len() / 2;
        let (c1, c2) = (&s[..midpoint], &s[midpoint..]);

        Rucksack {
            compartments_def: s,
            compartments: [Compartment::from_str(c1), Compartment::from_str(c2)],
        }
    }

    pub fn overlapping_item(&self) -> char {
        let (left, right) = (&self.compartments[0], &self.compartments[1]);
        let left_items: HashSet<char> = HashSet::from_iter(left.as_str().chars());
        right
            .as_str()
            .chars()
            .find(|ch| left_items.contains(ch))
            .expect("no overvlap in compartment items")
    }

    pub fn as_str(&self) -> &str {
        self.compartments_def
    }

    pub fn items(&self) -> HashSet<char> {
        HashSet::from_iter(self.as_str().chars())
    }
}

pub struct Compartment<'a> {
    items_def: &'a str,
}

impl<'a> Compartment<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Compartment { items_def: s }
    }

    pub fn as_str(&self) -> &'a str {
        self.items_def
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compartment_splitting() {
        let input = "ABCDabcd";
        let rucksack = Rucksack::from_str(input);
        assert!(rucksack.compartments[0].as_str().len() == 4);
        assert!(rucksack.compartments[1].as_str().len() == 4);
    }

    #[test]
    #[should_panic]
    fn overlap_finding_fails() {
        let input = "ABCDabcd";
        let rucksack = Rucksack::from_str(input);
        rucksack.overlapping_item();
    }

    #[test]
    fn overlap_finding_succeeds() {
        let rucksack = Rucksack::from_str("abcXYa");
        assert!(rucksack.overlapping_item() == 'a');
        let rucksack = Rucksack::from_str("abcXcY");
        assert!(rucksack.overlapping_item() == 'c');
    }
}
