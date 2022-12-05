use super::Movement;
use std::collections::VecDeque;

pub struct DeckBuilder(Vec<VecDeque<char>>);

impl DeckBuilder {
    pub fn new() -> Self {
        DeckBuilder(vec![])
    }

    pub fn feed_line(&mut self, line: &str) {
        line.chars()
            .chain([' ']) // dirty hack to account for the final
            // [x] *not* including a trailing space
            .array_chunks::<4>()
            .enumerate()
            .for_each(|(idx, chars)| {
                if chars[1] != ' ' {
                    self.ensure_stacks_init(idx + 1);
                    self.0[idx].push_back(chars[1]);
                }
            });
    }

    pub fn build(self) -> Deck {
        Deck(self.0)
    }

    fn ensure_stacks_init(&mut self, n: usize) {
        let stacks_needed = n.saturating_sub(self.0.len());
        for _ in 0..stacks_needed {
            self.0.push(VecDeque::new());
        }
    }
}

#[derive(Debug)]
pub struct Deck(Vec<VecDeque<char>>);

impl Deck {
    pub fn make_movement_9000(&mut self, m: &Movement) {
        for _ in 0..m.count {
            let crate_ = self.0[m.source - 1].pop_front().unwrap();
            self.0[m.destination - 1].push_front(crate_);
        }
    }

    pub fn make_movement_9001(&mut self, m: &Movement) {
        let crates_not_moving = self.0[m.source].len() - m.count;
        let (source, destination) = self.mut_refs_to(m.source, m.destination);
        for ch in source.iter().skip(crates_not_moving) {
            destination.push_front(*ch);
        }
        source.truncate(crates_not_moving);
    }

    pub fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.front().unwrap_or(&' '))
            .collect()
    }

    /// It's a bit ugly bug rust can't properly understand it's fine for two separate `&mut`
    /// borrows to two distinct elements in a Vec/Array. `split_at_mut` uses unsafe under the
    /// hood to get around this limitation. Unfortunately that means since we want two specific
    /// indices, not two separate splits of `self.0`, we need to do some juggling of indicies.
    fn mut_refs_to(&mut self, i1: usize, i2: usize) -> (&mut VecDeque<char>, &mut VecDeque<char>) {
        assert!(i1 != i2);
        if i1 > i2 {
            let (start, end) = self.0.split_at_mut(i1);
            (&mut end[0], &mut start[i2])
        } else {
            let (start, end) = self.0.split_at_mut(i2);
            (&mut start[i1], &mut end[0])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Deck {
        pub fn inner_vec(&self) -> &Vec<VecDeque<char>> {
            &self.0
        }
    }

    #[test]
    fn make_movement_9001_copying() {
        let mut deck = {
            let mut builder = DeckBuilder::new();
            for line in [
                "[I] [J]        ",
                "[E] [F] [G] [H]",
                "[A] [B] [C] [D]",
            ] {
                builder.feed_line(line);
            }
            builder.build()
        };
        deck.make_movement_9001(&Movement {
            count: 2,
            source: 2,
            destination: 3,
        });
        let vecs = deck.inner_vec();
        assert!(vecs[1][0] == 'B');
        assert!(vecs[1].len() == 1);
        assert!(vecs[2][3] == 'J');
        assert!(vecs[2][2] == 'F');
        assert!(vecs[2][1] == 'G');
        assert!(vecs[2][0] == 'C');
        assert!(vecs[1].len() == 4);
    }
}
