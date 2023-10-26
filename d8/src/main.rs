use anyhow::Context;
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;

    let visible_trees_count = parsed
        .iter()
        .filter(|tree| tree.is_visible().expect("failed visible check"))
        .count();

    println!("Visible trees:");
    println!("{}", visible_trees_count);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use crate::input_parser;

    #[test]
    fn given_example_part_1() {
        let input_file = File::open("./example.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let visible_trees_count = parsed
            .iter()
            .filter(|tree| tree.is_visible().expect("failed visible check"))
            .count();

        assert_eq!(21, visible_trees_count);
    }
}
