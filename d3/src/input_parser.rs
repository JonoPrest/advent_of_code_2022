use anyhow::{anyhow, Context};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Item = char;
type Priority = i32;
type Items = Vec<Item>;

#[derive(Debug, Clone)]
pub struct Rucksack {
    pub items: Items,
}

pub struct PriorityMap {
    map: HashMap<Item, Priority>,
}

impl PriorityMap {
    pub fn new() -> Self {
        let lower_alphabet_with_vals: Vec<(Item, Priority)> = ('a'..='z').zip(1..=26).collect();
        let upper_alphabet_with_vals: Vec<(Item, Priority)> = ('A'..='Z').zip(27..=52).collect();

        let mut map = HashMap::new();

        for i in 0..26 {
            map.insert(lower_alphabet_with_vals[i].0, lower_alphabet_with_vals[i].1);
            map.insert(upper_alphabet_with_vals[i].0, upper_alphabet_with_vals[i].1);
        }

        PriorityMap { map }
    }

    pub fn try_get_item_priority(&self, item: &Item) -> anyhow::Result<&Priority> {
        self.map
            .get(item)
            .ok_or_else(|| anyhow!("Item '{}' does not have a value", item))
    }
}

impl Rucksack {
    pub fn get_first_comparment(&self) -> Items {
        self.items[0..self.items.len() / 2]
            .iter()
            .cloned()
            .collect()
    }
    pub fn get_second_comparment(&self) -> Items {
        self.items[(self.items.len() / 2)..self.items.len()]
            .iter()
            .cloned()
            .collect()
    }

    pub fn get_common_item(&self) -> Item {
        *self
            .get_first_comparment()
            .iter()
            .find(|&item| {
                self.get_second_comparment()
                    .iter()
                    .find(|&item2| item == item2)
                    .is_some()
            })
            .expect("should be a common item in each compartment")
    }
}
pub type RucksackItems = Vec<Rucksack>;

#[derive(Debug, Clone)]
pub struct ElfGroup {
    group: RucksackItems,
}

impl ElfGroup {
    pub fn get_group_badge(&self) -> Item {
        self.group
            .iter()
            .enumerate()
            .find_map(|(i, elf)| {
                elf.items
                    .iter()
                    .find(|&item| {
                        self.group.iter().enumerate().all(|(i2, elf2)| {
                            if i2 == i {
                                true
                            } else {
                                elf2.items.iter().find(|&item2| item2 == item).is_some()
                            }
                        })
                    })
                    .cloned()
            })
            .expect("Unable to find group badeg")
    }
}

pub type ElfGroups = Vec<ElfGroup>;

pub fn subdivide_into_elf_groups(elfs: RucksackItems) -> ElfGroups {
    let mut elf_groups = Vec::new();
    let mut group = Vec::new();

    for elf in elfs {
        group.push(elf);
        if group.len() >= 3 {
            elf_groups.push(ElfGroup {
                group: group.clone(),
            });
            group = Vec::new();
        }
    }
    elf_groups
}

pub fn parse_input(file: &File) -> anyhow::Result<RucksackItems> {
    let reader = BufReader::new(file);
    let mut rucksacks: RucksackItems = Vec::new();
    for line in reader.lines() {
        let line = line.context("reading line to string")?;
        let content = line.trim();

        let items = content.chars().collect();
        rucksacks.push(Rucksack { items });
    }

    Ok(rucksacks)
}
