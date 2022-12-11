use std::collections::VecDeque;

use crate::{operation, test};

pub struct Monkey {
    pub items: VecDeque<usize>,
    pub inspections: usize,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    throw_true: usize,
    throw_false: usize,
}

impl From<&mut dyn Iterator<Item = String>> for Monkey {
    fn from(iter: &mut dyn Iterator<Item = String>) -> Self {
        let _ = iter.next().unwrap(); // drop `Monkey n:` line

        let items = iter.next().unwrap();
        assert!(items.starts_with("  Starting items: "));
        let items = &items[18..];
        let items: VecDeque<usize> = items
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        let op = iter.next().unwrap();
        assert!(op.starts_with("  Operation: "));
        let op = &op[13..];
        let op = operation::parse(op);

        let test = iter.next().unwrap();
        assert!(test.starts_with("  Test: "));
        let test = &test[8..];
        let test = test::parse(test);

        let throw_true = iter.next().unwrap();
        assert!(throw_true.starts_with("    If true: throw to monkey "));
        let throw_true = *&throw_true[29..].parse::<usize>().unwrap();

        let throw_false = iter.next().unwrap();
        assert!(throw_false.starts_with("    If false: throw to monkey "));
        let throw_false = *&throw_false[30..].parse::<usize>().unwrap();

        Self {
            items,
            inspections: 0,
            op,
            test,
            throw_true,
            throw_false,
        }
    }
}

impl Monkey {
    /// Inspect the next item, if any, and return the item
    /// and to which Monkey the item should be thrown.
    pub fn inspect(&mut self) -> Option<(usize, usize)> {
        let mut item = self.items.pop_front()?;
        item = (self.op)(item);
        item = (item as f64 / 3.0).floor() as usize;
        let test = (self.test)(item);

        self.inspections += 1;
        Some((
            item,
            if test {
                self.throw_true
            } else {
                self.throw_false
            },
        ))
    }
}
