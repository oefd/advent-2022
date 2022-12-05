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
    pub fn make_movement(&mut self, m: &Movement) {
        for _ in 0..m.count {
            let crate_ = self.0[m.source - 1].pop_front().unwrap();
            self.0[m.destination - 1].push_front(crate_);
        }
    }

    pub fn top_crates(&self) -> String {
        self.0.iter().map(|stack| stack.front().unwrap_or(&' ')).collect()
    }
}
