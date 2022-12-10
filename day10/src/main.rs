mod isa;

use isa::{Instruction, SideEffect};

fn main() {
    let stdin = util::Stdin::new();

    let mut cpu = Cpu::<1>::new();
    let mut cycle_of_interest = cycles_of_interest(220);
    let mut wait_for_cycle = cycle_of_interest.next().unwrap();
    let mut cumulative_strength = 0;
    let mut lines = stdin.cleaned_lines();
    'run_cpu: loop {
        tick_until_clock_or_complete_instruction(&mut cpu, wait_for_cycle);
        if cpu.i.is_none() {
            if let Some(line) = lines.next() {
                cpu.i = Some(Instruction::from(line.as_str()));
            } else {
                break 'run_cpu;
            }
        }
        if cpu.clock == wait_for_cycle {
            wait_for_cycle = cycle_of_interest.next().unwrap();
            let signal_strength = (cpu.clock as isize) * cpu.r[0];
            println!("at clock {} strength is {}", cpu.clock, signal_strength);
            cumulative_strength += signal_strength;
        }
    }
    println!("cumulative strength: {}", cumulative_strength);
}

fn tick_until_clock_or_complete_instruction<const N: usize>(cpu: &mut Cpu<N>, clock: usize) {
    loop {
        if cpu.i.is_some() && cpu.clock < clock {
            cpu.tick();
        } else {
            break;
        }
    }
}

fn cycles_of_interest(max: usize) -> impl Iterator<Item = usize> {
    let mut cycle = 0;
    std::iter::from_fn(move || {
        if cycle == 0 {
            cycle = 20;
            Some(20)
        } else {
            cycle += 40;
            if cycle > max {
                Some(usize::MAX)
            } else {
                Some(cycle)
            }
        }
    })
}

#[derive(Debug)]
struct Cpu<const N: usize> {
    pub r: [isize; N],
    pub i: Option<Instruction>,
    pub clock: usize,
}

impl<const N: usize> Cpu<N> {
    pub fn new() -> Self {
        Cpu {
            clock: 1,
            r: [1; N],
            i: None,
        }
    }

    /// Advance the clock by one cycle, returning whether the instruction completed
    pub fn tick(&mut self) {
        let mut instruction = self.i.take().unwrap();
        let (effect, is_done) = instruction.tick();
        self.apply_effect(effect);
        self.clock += 1;

        if !is_done {
            self.i = Some(instruction);
        }
    }

    fn apply_effect(&mut self, side_effect: SideEffect) {
        match side_effect {
            SideEffect::None => {}
            SideEffect::Addx { r, x } => {
                self.r[r] += x;
            }
        };
    }
}
