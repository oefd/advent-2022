//! Represents a round as a set of hands, and provides Into/From implementations
//! to covert between these types, to create these types from strs, and convert
//! these types to player scores.
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Into<u64> for Hand {
    fn into(self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Hand {
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
            (any_hand, Draw) => any_hand.clone(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Victor {
    Me,
    Them,
    Draw,
}

impl From<Round> for Victor {
    fn from(round: Round) -> Self {
        use Hand::{Paper, Rock, Scissors};
        use Victor::{Draw, Me, Them};
        match (round.me, round.them) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Paper) => Them,
            (Rock, Scissors) => Me,
            (Paper, Rock) => Me,
            (Paper, Scissors) => Them,
            (Scissors, Rock) => Them,
            (Scissors, Paper) => Me,
        }
    }
}

impl Into<u64> for Victor {
    fn into(self) -> u64 {
        use Victor::{Draw, Me, Them};
        match self {
            Me => 6,
            Draw => 3,
            Them => 0,
        }
    }
}

impl Victor {
    pub fn desired(ch: &u8) -> Self {
        use Victor::{Draw, Me, Them};
        match ch {
            b'X' => Them,
            b'Y' => Draw,
            b'Z' => Me,
            other => panic!("invalid victory target {}", other),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Round {
    me: Hand,
    them: Hand,
}

impl Into<u64> for Round {
    fn into(self) -> u64 {
        let victory_score: u64 = Victor::from(self).into();
        let hand_score: u64 = self.me.into();

        victory_score + hand_score
    }
}

impl FromStr for Round {
    type Err = Infallible;

    /// This is only valid for part 1
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() == 3);
        let s = s.as_bytes();

        let them = match s[0] {
            b'A' => Hand::Rock,
            b'B' => Hand::Paper,
            b'C' => Hand::Scissors,
            other => panic!("invalid hand for other player: {}", other),
        };
        let me = match s[2] {
            b'X' => Hand::Rock,
            b'Y' => Hand::Paper,
            b'Z' => Hand::Scissors,
            other => panic!("invalid hand for me to play: {}", other),
        };

        Ok(Round { me, them })
    }
}

impl Round {
    /// This would properly be a `FromStr` implementation, but we're already
    /// using that implemenation for the part 1 interpretation of the X/Y/Z
    pub fn from_str_part_2(s: &str) -> Self {
        assert!(s.len() == 3);
        let s = s.as_bytes();

        let them = match s[0] {
            b'A' => Hand::Rock,
            b'B' => Hand::Paper,
            b'C' => Hand::Scissors,
            other => panic!("invalid hand for other player: {}", other),
        };

        let target = Victor::desired(&s[2]);
        let me = Hand::required_response(&them, &target);
        Round { me, them }
    }
}
