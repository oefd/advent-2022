fn main() {
    let input = std::io::stdin().lines().map(Result::unwrap);
    #[cfg(feature = "part_1")]
    {
        let count = top_elf_count(input);
        println!("top elf: {}", count);
    }

    #[cfg(feature = "part_2")]
    {
        let count = top_elves_count(input);
        println!("top elves: {}", count);
    }
}

//
// Attempt at vaguely idiomatic (still heavily defined by my personal preferences)
// solutions, not going out of my way to try and be performant.
//

#[cfg(not(feature = "attempt_optimal"))]
fn counts<I: Iterator<Item = String>>(input: I) -> Vec<u64> {
    let lines = input
        // ensure we have a final line since we rely on
        // that sigil for terminating an elf's list
        .chain(["".to_string()]);
    lines
        .fold((Vec::new(), 0u64), |(mut counts, acc), line| {
            if line.is_empty() {
                counts.push(acc);
                (counts, 0)
            } else {
                let cals_in_item: u64 = line.parse().unwrap();
                (counts, acc + cals_in_item)
            }
        })
        .0
}

#[cfg(not(feature = "attempt_optimal"))]
fn top_elf_count<I: Iterator<Item = String>>(input: I) -> u64 {
    counts(input).into_iter().max().unwrap_or(0)
}

#[cfg(not(feature = "attempt_optimal"))]
fn top_elves_count<I: Iterator<Item = String>>(input: I) -> u64 {
    let mut counts = counts(input);
    counts.sort_unstable();
    counts.into_iter().rev().take(3).sum()
}

//
// Attempt at sanic level gotta go fast code
//

#[cfg(feature = "attempt_optimal")]
fn top_elf_count<I: Iterator<Item = String>>(input: I) -> u64 {
    let mut acc = 0;
    let mut highest = 0;

    for line in input
        // ensure we have a final line since we rely on
        // that sigil for terminating an elf's list
        .chain(["".to_string()])
    {
        if line.is_empty() {
            highest = std::cmp::max(highest, acc);
            acc = 0;
        } else {
            acc += line.parse::<u64>().unwrap();
        }
    }
    highest
}

#[cfg(feature = "attempt_optimal")]
fn top_elves_count<I: Iterator<Item = String>>(input: I) -> u64 {
    let mut acc = 0;
    let mut highest_hi = 0;
    let mut highest_md = 0;
    let mut highest_lo = 0;
    for line in input
        // ensure we have a final line since we rely on
        // that sigil for terminating an elf's list
        .chain(["".to_string()])
    {
        if line.is_empty() {
            if acc > highest_hi {
                highest_lo = highest_md;
                highest_md = highest_hi;
                highest_hi = acc;
            } else if acc > highest_md {
                highest_lo = highest_md;
                highest_md = acc;
            } else if acc > highest_lo {
                highest_lo = acc;
            }
            acc = 0;
        } else {
            acc += line.parse::<u64>().unwrap();
        }
    }
    highest_hi + highest_md + highest_lo
}
