use anyhow::{anyhow, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::rope_simulator::{Direction, Motion};

type Motions = Vec<Motion>;

pub fn parse_input(file: &File) -> Result<Motions> {
    let reader = BufReader::new(file);
    let motions: Motions = reader
        .lines()
        .map(|line| {
            let content = line?;
            let line_chars = content.chars().collect::<Vec<_>>();

            let direction = match line_chars
                .first()
                .ok_or_else(|| anyhow!("No first char in line"))?
            {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                bad_char => Err(anyhow!("Non direction char, {}", bad_char))?,
            };

            let amount = line_chars
                .get(2)
                .ok_or_else(|| anyhow!("No amountchar in line"))?
                .to_digit(10)
                .ok_or_else(|| anyhow!("Char is not a digit"))? as i32;

            Ok(Motion { direction, amount })
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("parsing motions from reader")?;

    Ok(motions)
}

#[cfg(test)]
mod test {}
