use anyhow::Context;
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;

    let num_fully_contained = parsed
        .iter()
        .filter(|pair| pair.range_is_fully_contained_in_other())
        .count();

    println!("num_fully_contained {}", num_fully_contained);

    //Part 2
    let num_overlap = parsed
        .iter()
        .filter(|pair| pair.range_contains_overlap())
        .count();

    println!("num_overlap {}", num_overlap);
    Ok(())
}
