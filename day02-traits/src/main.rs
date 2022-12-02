use std::str::FromStr;

mod round;

use round::Round;

fn main() {
    let stdin = util::Stdin::new();

    let score = part_1_score(stdin.lines());
    println!("part 1 score: {}", score);

    let score = part_2_score(stdin.lines());
    println!("part 2 score: {}", score);
}

fn part_1_score<I>(input: I) -> u64
where
    I: Iterator<Item = String>,
{
    input.fold(0, |acc_score, line| {
        if line.is_empty() {
            acc_score
        } else {
            let round = Round::from_str(&line).unwrap();
            let round_score: u64 = round.into();
            acc_score + round_score
        }
    })
}

fn part_2_score<I>(input: I) -> u64
where
    I: Iterator<Item = String>,
{
    input.fold(0, |acc_score, line| {
        if line.is_empty() {
            acc_score
        } else {
            let round = Round::from_str_part_2(&line);
            let round_score: u64 = round.into();
            acc_score + round_score
        }
    })
}
