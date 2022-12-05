#[derive(Debug)]
pub struct Movement {
    pub count: usize,
    pub source: usize,
    pub destination: usize,
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let mut splits = s.split_whitespace();
        splits.next();
        let count = splits.next().unwrap().parse().unwrap();
        splits.next();
        let source = splits.next().unwrap().parse().unwrap();
        splits.next();
        let destination = splits.next().unwrap().parse().unwrap();

        Movement {
            count,
            source,
            destination,
        }
    }
}
