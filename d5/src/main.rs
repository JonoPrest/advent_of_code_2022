use anyhow::Context;
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;
    // println!("parsed {:?}", parsed);
    //
    let mut ship = parsed.ship.clone();

    for instruction in parsed.instructions.clone() {
        ship.apply_instruction(instruction)
            .context("apply_instruction")?;
    }

    let message = ship.get_top_message();
    println!("Message: {}", message);

    //Part 2

    let mut ship_2 = parsed.ship.clone();

    for instruction in parsed.instructions {
        ship_2
            .apply_instruction_multi(instruction)
            .context("apply_instruction_multi")?;
    }

    let message = ship_2.get_top_message();
    println!("Message 2: {}", message);

    Ok(())
}
