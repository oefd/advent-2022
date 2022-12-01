use std::io::{BufRead, Cursor, Write};

/// Wrapper of stdin to do some convenience cloning/unwrapping that make
/// throwaway advent code challenges a bit more simple/terse.
#[derive(Default)]
pub struct Stdin {
    cursor: Cursor<Vec<u8>>,
}

impl Stdin {
    pub fn new() -> Self {
        let mut stdin = Cursor::new(Vec::new());
        for line in std::io::stdin().lines().map(Result::unwrap) {
            stdin.write(line.as_bytes()).unwrap();
            stdin.write(b"\n").unwrap();
        }
        stdin.set_position(0);
        Stdin { cursor: stdin }
    }

    pub fn lines(&self) -> impl Iterator<Item = String> {
        self.cursor.clone().lines().map(Result::unwrap)
    }
}
