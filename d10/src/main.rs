use anyhow::Context;
use std::fs::File;

use crate::input_parser::Signal;

mod input_parser;

fn main() -> anyhow::Result<()> {
    let input_file = File::open("./input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file);

    // println!("parsed: {:?}", parsed);

    let mut program = ProgramExecution::new();
    program.execute_signals(parsed);

    let signal_strength: i32 = (0..6)
        .into_iter()
        .map(|i| (i * 40) + 20)
        .filter_map(|cycle| program.get_signal_strength_at_cycle(cycle))
        .sum();

    println!("signal strength: ");
    println!("{signal_strength}");

    Ok(())
}

struct ProgramExecution {
    x: i32,
    cycles: Vec<i32>,
}

impl ProgramExecution {
    fn new() -> Self {
        Self {
            x: 1,
            cycles: Vec::new(),
        }
    }

    fn execute_signal(&mut self, signal: Signal) {
        match signal {
            Signal::NoOp => self.cycles.push(self.x),
            Signal::AddX(v) => {
                self.cycles.extend(vec![self.x, self.x]);
                self.x = self.x + v;
            }
        }
    }

    pub fn execute_signals(&mut self, signals: Vec<Signal>) {
        for signal in signals {
            self.execute_signal(signal);
        }
    }

    pub fn get_x_at_cycle(&self, cycle: usize) -> Option<&i32> {
        self.cycles.get(cycle - 1)
    }

    pub fn get_signal_strength_at_cycle(&self, cycle: usize) -> Option<i32> {
        self.get_x_at_cycle(cycle).map(|x| x * (cycle as i32))
    }
}
