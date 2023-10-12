use anyhow::Context;
use std::fs::File;

use crate::input_parser::Round;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    let parsed_one = input_parser::parse_input_first(&input_file).context("parsing file")?;

    let total_score: i32 = parsed_one.iter().map(|round| round.to_player_score()).sum();

    println!("player fist total: {:?}", total_score);

    let input_file = File::open("./my_input.txt").context("opening file")?;

    let parsed_two = input_parser::parse_input_second(&input_file).context("parsing file")?;

    let rounds_two: Vec<_> = parsed_two
        .iter()
        .map(|round_strat| Round::from(round_strat.clone()))
        .collect();

    let total_score_two: i32 = rounds_two.iter().map(|round| round.to_player_score()).sum();

    println!("player second total: {:?}", total_score_two);

    Ok(())
}
