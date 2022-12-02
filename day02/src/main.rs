fn main() {
    let stdin = util::Stdin::new();

    let mut part_1_score = 0;
    for line in stdin.lines() {
        if line.is_empty() {
            continue;
        }
        let round = Round::from_line_part_1(&line);
        part_1_score += round.score_value();
    }
    println!("part 1 score: {}", part_1_score);

    let mut part_2_score = 0;
    for line in stdin.lines() {
        if line.is_empty() {
            continue;
        }
        let round = Round::from_line_part_2(&line);
        part_2_score += round.score_value();
    }
    println!("part 2 score: {}", part_2_score);
}

struct Round {
    me: Hand,
    them: Hand,
}

impl Round {
    pub fn from_line_part_1(line: &str) -> Self {
        assert!(line.len() == 3);
        let line = line.as_bytes();
        Self {
            me: Hand::from_second_column(&line[2]),
            them: Hand::from_first_column(&line[0]),
        }
    }

    pub fn from_line_part_2(line: &str) -> Self {
        assert!(line.len() == 3);
        let line = line.as_bytes();
        let them = Hand::from_first_column(&line[0]);
        let target_outcome = Victor::desired_from_second_column(&line[2]);
        let me = Hand::required_response(&them, &target_outcome);
        Self { me, them }
    }

    pub fn score_value(&self) -> u64 {
        let victor = Victor::from_round(self);
        victor.score_value() + self.me.score_value()
    }
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn from_first_column(byte: &u8) -> Self {
        match byte {
            b'A' => Self::Rock,
            b'B' => Self::Paper,
            b'C' => Self::Scissors,
            other => panic!("invalid first column byte {}", other),
        }
    }

    pub fn from_second_column(byte: &u8) -> Self {
        match byte {
            b'X' => Self::Rock,
            b'Y' => Self::Paper,
            b'Z' => Self::Scissors,
            other => panic!("invalid second column byte {}", other),
        }
    }

    pub fn required_response(them: &Hand, desired_outcome: &Victor) -> Hand {
        use Hand::{Paper, Rock, Scissors};
        use Victor::{Draw, Me, Them};
        match (them, desired_outcome) {
            (Rock, Me) => Paper,
            (Rock, Them) => Scissors,
            (Paper, Me) => Scissors,
            (Paper, Them) => Rock,
            (Scissors, Me) => Rock,
            (Scissors, Them) => Paper,
            // It's possible to condense the Draw cases in to
            // (any, Draw) => any.clone()
            // with #[derive(Copy, Clone)] on `Hand`
            (Rock, Draw) => Rock,
            (Paper, Draw) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }

    pub fn score_value(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Victor {
    Me,
    Them,
    Draw,
}

impl Victor {
    pub fn from_round(round: &Round) -> Victor {
        use Hand::{Paper, Rock, Scissors};
        match (&round.me, &round.them) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Self::Draw,
            (Rock, Paper) => Self::Them,
            (Rock, Scissors) => Self::Me,
            (Paper, Rock) => Self::Me,
            (Paper, Scissors) => Self::Them,
            (Scissors, Rock) => Self::Them,
            (Scissors, Paper) => Self::Me,
        }
    }

    pub fn desired_from_second_column(byte: &u8) -> Self {
        match byte {
            b'X' => Self::Them,
            b'Y' => Self::Draw,
            b'Z' => Self::Me,
            other => panic!("invalid desired victory outcome {}", other),
        }
    }

    pub fn score_value(&self) -> u64 {
        match self {
            Self::Them => 0,
            Self::Draw => 3,
            Self::Me => 6,
        }
    }
}
