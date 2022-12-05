use std::ops::RangeInclusive;

fn main() {
    let stdin = util::Stdin::new();
    let lines = stdin.cleaned_lines();
    let pairs: Vec<AssignmentPair> = lines
        .map(|line| AssignmentPair::from(line.as_str()))
        .collect();

    // part 1
    let envelope_count = pairs.iter().filter(|pair| pair.is_envelopment()).count();
    println!("{} assignment pairs include an envelopment", envelope_count);

    // part 2
    let overlap_count = pairs.iter().filter(|pair| pair.is_overlap()).count();
    println!("{} assignment pairs include an overlap", overlap_count);
}

struct AssignmentPair {
    pub first: RangeInclusive<usize>,
    pub second: RangeInclusive<usize>,
}

impl From<&str> for AssignmentPair {
    fn from(s: &str) -> Self {
        let (first, second) = s.split_once(',').unwrap();
        AssignmentPair {
            first: Self::parse_assignment(first),
            second: Self::parse_assignment(second),
        }
    }
}

impl AssignmentPair {
    fn parse_assignment(s: &str) -> RangeInclusive<usize> {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
        start..=end
    }

    fn is_envelopment(&self) -> bool {
        let (first, second) = (&self.first, &self.second);
        let first_envelops_second =
            first.contains(&second.start()) && first.contains(&second.end());
        let second_envelops_first =
            second.contains(&first.start()) && second.contains(&first.end());
        first_envelops_second || second_envelops_first
    }

    fn is_overlap(&self) -> bool {
        let (first, second) = (&self.first, &self.second);
        first.contains(second.start()) || first.contains(second.end()) || self.is_envelopment()
    }
}
