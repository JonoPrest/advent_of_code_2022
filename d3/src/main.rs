use anyhow::Context;
use std::fs::File;

use crate::input_parser::{subdivide_into_elf_groups, PriorityMap};
mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed_rucksacks = input_parser::parse_input(&input_file).context("parsing file")?;

    let priority_map = PriorityMap::new();

    let total_priority: i32 = parsed_rucksacks
        .iter()
        .map(|rucksack| priority_map.try_get_item_priority(&rucksack.get_common_item()))
        .collect::<anyhow::Result<Vec<_>>>()
        .context("getting priority")?
        .into_iter()
        .sum();

    println!("total priority: {}", total_priority);

    //Part 2
    let elf_groups = subdivide_into_elf_groups(parsed_rucksacks);

    let group_badge_total_priority: i32 = elf_groups
        .iter()
        .map(|group| priority_map.try_get_item_priority(&group.get_group_badge()))
        .collect::<anyhow::Result<Vec<_>>>()
        .context("getting priority")?
        .into_iter()
        .sum();

    println!("group badge total priority {}", group_badge_total_priority);

    Ok(())
}
