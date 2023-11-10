use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub enum Signal {
    AddX(i32),
    NoOp,
}
fn parse_addx(input: &str) -> IResult<&str, Signal> {
    let (input, _) = tag("addx")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = i32(input)?;
    Ok((input, Signal::AddX(value)))
}

fn parse_noop(input: &str) -> IResult<&str, Signal> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Signal::NoOp))
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    alt((parse_addx, parse_noop))(input)
}

type SignalRegister = Vec<Signal>;
pub fn parse_input(file: &File) -> SignalRegister {
    let reader = BufReader::new(file);
    let signals: SignalRegister = reader
        .lines()
        .map(|line| {
            let content = line.expect("parsing");
            let (_, signal) = parse_signal(content.as_str()).expect("parsing");
            signal
        })
        .collect();

    signals
}

#[cfg(test)]
mod test {}
