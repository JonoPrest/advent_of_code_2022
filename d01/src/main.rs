use anyhow::Context;
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    let parsed = input_parser::get_elfs_from_file(&input_file).context("parsing file")?;

    let mut summed: Vec<i32> = parsed
        .iter()
        .map(|elf_cals| elf_cals.iter().cloned().sum())
        .collect();

    //part one get the highest
    let highest = summed.iter().max().context("getting the max")?;

    println!("highest elf cals {:?}", highest);

    //part 2
    let mut top_three = Vec::new();

    while top_three.len() < 3 {
        let (i, _) = summed
            .iter()
            .enumerate()
            .max_by_key(|(_, &x)| x)
            .expect("expected max");

        let max = summed.remove(i);
        top_three.push(max);
    }

    println!("top 3 = {:?}", top_three);

    let top_three_summed: i32 = top_three.iter().sum();

    println!("total = {:?}", top_three_summed);

    Ok(())
}
