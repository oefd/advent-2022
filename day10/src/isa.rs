#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx { r: usize, x: isize, cycles: usize },
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Self::noop()
        } else if s.starts_with("addx") {
            let (_, x) = s.split_once(' ').unwrap();
            let x = x.parse::<isize>().unwrap();
            Self::addx(0, x)
        } else {
            panic!("invalid instruction {}", s)
        }
    }
}

impl Instruction {
    pub fn noop() -> Self {
        Self::Noop
    }

    pub fn addx(r: usize, x: isize) -> Self {
        Self::Addx { r, x, cycles: 2 }
    }

    /// Advance the clock by one cycle, returning the side
    /// effect and whether this instruction is now completed.
    pub fn tick(&mut self) -> (SideEffect, bool) {
        match self {
            Self::Noop => (SideEffect::None, true),
            Self::Addx {
                r,
                x,
                ref mut cycles,
            } => {
                *cycles -= 1;
                if *cycles == 0 {
                    (SideEffect::Addx { r: *r, x: *x }, true)
                } else {
                    (SideEffect::None, false)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum SideEffect {
    None,
    Addx {r: usize, x: isize},
}
