use anyhow::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub type ItemCalories = i32;
pub type ElfItems = Vec<ItemCalories>;
pub type Elfs = Vec<ElfItems>;

pub fn get_elfs_from_file(file: &File) -> anyhow::Result<Elfs> {
    let reader = BufReader::new(file);
    let mut elfs: Elfs = Vec::new();
    let mut current_elf: ElfItems = Vec::new();
    for line in reader.lines() {
        let line = line.context("reading line to string")?;
        let content = line.trim();
        match content {
            "" => {
                elfs.push(current_elf.clone());
                current_elf = Vec::new();
            }
            value => {
                let numeric_val: ItemCalories = value.parse().context("parsing value")?;
                current_elf.push(numeric_val);
            }
        }
    }

    Ok(elfs)
}
