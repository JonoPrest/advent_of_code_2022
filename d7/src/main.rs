use anyhow::{anyhow, Context};
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;
    println!("parsed {:?}", parsed);

    Ok(())
}
