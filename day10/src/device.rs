pub struct Device<T: Iterator<Item = Op>> {
    pub r0: isize,
    pub clock: usize,
    pipeline: T,
    crt: [char; 40 * 6],
    pc: Op,
}

impl<T: Iterator<Item = Op>> Device<T> {
    pub fn new(mut pipeline: T) -> Self {
        let pc = pipeline.next().unwrap();
        Self {
            r0: 1,
            clock: 1,
            pipeline,
            crt: ['.'; 40 * 6],
            pc,
        }
    }

    /// Returns `false` when no progress can be made
    /// a due to lack of further ops to execute.
    pub fn tick(&mut self) -> bool {
        self.draw_pixel();
        self.clock += 1;

        match self.pc {
            Op::Noop => self.load_op(),
            Op::Addx {
                ref mut cycles,
                value,
            } => {
                *cycles -= 1;
                if *cycles == 0 {
                    self.r0 += value;
                    self.load_op()
                } else {
                    true
                }
            }
            Op::Halt => false,
        }
    }

    pub fn draw_crt(&self) {
        println!();
        for i in 0..6 {
            for j in 0..40 {
                print!("{}", self.crt[(i * 40) + j]);
            }
            println!();
        }
        println!();
    }

    fn load_op(&mut self) -> bool {
        self.pc = self.pipeline.next().unwrap_or(Op::Halt);
        !matches!(self.pc, Op::Halt)
    }

    fn draw_pixel(&mut self) {
        let sprite = (self.r0 - 1)..=(self.r0 + 1);
        let pixel = (self.clock - 1) % 240;
        let pixel_on = if sprite.contains(&((pixel as isize) % 40)) {
            '#'
        } else {
            '.'
        };
        self.crt[pixel] = pixel_on;
    }
}

pub enum Op {
    Noop,
    Halt,
    Addx { cycles: isize, value: isize },
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        if s == "noop" {
            Op::Noop
        } else if s.starts_with("addx") {
            let (_, value) = s.split_once(' ').unwrap();
            let value = value.parse::<isize>().unwrap();
            Op::Addx { cycles: 2, value }
        } else {
            panic!("invalid op {}", s);
        }
    }
}
