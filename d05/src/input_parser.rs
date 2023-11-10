use anyhow::{anyhow, Context};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

type CrateStack = VecDeque<char>;

#[derive(Debug, Clone)]
pub struct Ship(Vec<CrateStack>);

impl Ship {
    pub fn apply_instruction(&mut self, instruction: Instruction) -> anyhow::Result<()> {
        // println!("Ship: {:?} \
        //     instruction: {:?}", self.0, instruction);
        for _ in 0..instruction.move_amount {
            let from_stack = self
                .0
                .get_mut(instruction.from - 1)
                .ok_or_else(|| anyhow!("failed to get stack {}", instruction.from))?;

            let val = from_stack
                .pop_front()
                .ok_or_else(|| anyhow!("no val on queue"))?;
            // if let Some(val) = from_stack.pop_front() {
            let to_stack = self
                .0
                .get_mut(instruction.to - 1)
                .ok_or_else(|| anyhow!("failed to get stack {}", instruction.to))?;

            to_stack.push_front(val);
            // }
        }
        Ok(())
    }

    pub fn apply_instruction_multi(&mut self, instruction: Instruction) -> anyhow::Result<()> {
        let mut intermediate_stack = VecDeque::new();
        for _ in 0..instruction.move_amount {
            let from_stack = self
                .0
                .get_mut(instruction.from - 1)
                .ok_or_else(|| anyhow!("failed to get stack {}", instruction.from))?;

            let val = from_stack
                .pop_front()
                .ok_or_else(|| anyhow!("no val on queue"))?;

            intermediate_stack.push_front(val);
        }

        for val in intermediate_stack.iter() {
            let to_stack = self
                .0
                .get_mut(instruction.to - 1)
                .ok_or_else(|| anyhow!("failed to get stack {}", instruction.to))?;

            to_stack.push_front(val.clone());
        }
        Ok(())
    }
    pub fn get_top_message(&self) -> String {
        self.0
            .iter()
            .filter_map(|stack| stack.front().cloned())
            .collect::<String>()
    }
}

type CrateStackId = usize;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub from: CrateStackId,
    pub to: CrateStackId,
    pub move_amount: i32,
}

type Instructions = Vec<Instruction>;

#[derive(Debug)]
pub struct ParsedInput {
    pub ship: Ship,
    pub instructions: Instructions,
}

pub fn parse_input(file: &File) -> anyhow::Result<ParsedInput> {
    let reader = BufReader::new(file);

    let mut ship_lines: VecDeque<String> = VecDeque::new();
    //extract out lines related to ship in a stack so they
    //can be parsed backwards from bottom up
    let mut instruction_lines: Vec<String> = Vec::new();
    let mut is_instruction = false;
    for line in reader.lines() {
        let content = line.context("reading line to string")?;

        if content.is_empty() {
            is_instruction = true;
        } else if !is_instruction {
            ship_lines.push_front(content.to_string());
        } else {
            instruction_lines.push(content);
        }
    }

    //parse lines related to ship
    let mut ship_crate_stacks: Vec<CrateStack> = Vec::new();

    if let Some(starter_line) = ship_lines.pop_front() {
        //In the first line we just want to instantiate
        //the stacks
        for _crate_stack_id in starter_line.trim().split("   ") {
            //_crate_stack_id is numbers 1 - 9 underlining the ship diagram
            ship_crate_stacks.push(VecDeque::new());
        }
    }

    for line in ship_lines.iter() {
        //The others we want to iterate through them and
        //push to their respective stacks
        let line_chars = line.chars().collect::<Vec<char>>();
        for (i_stack, stack_chars) in line_chars.chunks(4).enumerate() {
            if stack_chars.iter().all(|&char| char == ' ') {
                continue;
            }
            let crate_item = stack_chars
                .get(1)
                .cloned()
                .ok_or_else(|| anyhow!("Unable to get crate item"))?;

            let ship_crate_stack = ship_crate_stacks
                .get_mut(i_stack)
                .ok_or_else(|| anyhow!("Unable to get stack"))?;

            ship_crate_stack.push_front(crate_item);
        }
    }

    //Parse instructions
    let mut instructions: Instructions = Vec::new();

    for line in instruction_lines.iter() {
        let line_vals: Vec<&str> = line.split(" ").collect();

        let instruction = Instruction {
            move_amount: line_vals
                .get(1)
                .ok_or_else(|| anyhow!("getting move val"))?
                .parse()
                .context("parsing move val")?,
            from: line_vals
                .get(3)
                .ok_or_else(|| anyhow!("getting from val"))?
                .parse()
                .context("parsing from val")?,
            to: line_vals
                .get(5)
                .ok_or_else(|| anyhow!("getting to val"))?
                .parse()
                .context("parsing to val")?,
        };

        instructions.push(instruction)
    }

    Ok(ParsedInput {
        ship: Ship(ship_crate_stacks),
        instructions,
    })
}
