use std::collections::HashSet;
use std::hash::Hash;

pub struct RingBuf<T, const N: usize>
where
    T: Sized + Eq + Copy + Hash,
{
    cursor: usize,
    pub buf: [Option<T>; N],
}

impl<T, const N: usize> RingBuf<T, N>
where
    T: Sized + Eq + Copy + Hash,
{
    pub fn new() -> Self {
        Self {
            cursor: 0,
            buf: [None; N],
        }
    }

    pub fn write(&mut self, item: T) {
        self.buf[self.cursor] = Some(item);
        self.advance_cursor();
    }

    pub fn is_distinct_items(&self) -> bool {
        if self.buf.iter().any(|item| item.is_none()) {
            false
        } else {
            let items = self.buf.iter().copied().map(Option::unwrap);
            let set: HashSet<T> = HashSet::from_iter(items);
            set.len() == self.buf.len()
        }
    }

    fn advance_cursor(&mut self) {
        self.cursor += 1;
        if self.cursor == N {
            self.cursor = 0;
        }
    }
}
