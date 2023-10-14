use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

type AssignedRange = RangeInclusive<i32>;

#[derive(Debug)]
pub struct ElfPair(AssignedRange, AssignedRange);

impl ElfPair {
    pub fn range_is_fully_contained_in_other(&self) -> bool {
        self.0.clone().all(|item| self.1.contains(&item))
            || self.1.clone().all(|item| self.0.contains(&item))
    }

    pub fn range_contains_overlap(&self) -> bool {
        self.0.clone().any(|item| self.1.contains(&item))
    }
}

type ElfPairs = Vec<ElfPair>;

pub fn parse_input(file: &File) -> anyhow::Result<ElfPairs> {
    let reader = BufReader::new(file);
    let mut elf_pairs: ElfPairs = Vec::new();
    for line in reader.lines() {
        let line = line.context("reading line to string")?;

        let content = line.trim();

        let two_ranges: Vec<_> = content
            .split(",")
            .map(|ranges_str| {
                let two_numbers: Vec<i32> = ranges_str
                    .split("-")
                    .map(|num_str| Ok(num_str.parse()?))
                    .collect::<anyhow::Result<Vec<i32>>>()?;

                let lower_range = two_numbers
                    .get(0)
                    .cloned()
                    .ok_or_else(|| anyhow!("Failed to get lower number in range"))?;

                let upper_range = two_numbers
                    .get(1)
                    .cloned()
                    .ok_or_else(|| anyhow!("Failed to get upper number in range"))?;

                Ok(lower_range..=upper_range)
            })
            .collect::<anyhow::Result<_>>()
            .context(format!("Getting ranges from line {}", content))?;

        let first_in_pair = two_ranges
            .get(0)
            .cloned()
            .ok_or_else(|| anyhow!("Failed to get first elf in pair"))?;

        let second_in_pair = two_ranges
            .get(1)
            .cloned()
            .ok_or_else(|| anyhow!("Failed to get second elf in pair"))?;

        elf_pairs.push(ElfPair(first_in_pair, second_in_pair));
    }

    Ok(elf_pairs)
}
