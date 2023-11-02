use anyhow::Context;
use std::{collections::HashSet, fs::File};

mod input_parser;
mod rope_simulator;

use rope_simulator::{Coordinate, RopeMap};

fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;

    // println!("parsed: {:?}", parsed);

    let mut rope_map = RopeMap::init();

    for motion in parsed {
        rope_map.move_rope_n_times(motion.direction, motion.amount);
    }

    let no_unique_tail_positions = rope_map
        .path
        .into_iter()
        .map(|rco| rco.tail)
        .collect::<HashSet<Coordinate>>()
        .iter()
        .count();

    println!("No unique tail positions:");
    println!("{}", no_unique_tail_positions);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, fs::File};

    use crate::{
        input_parser,
        rope_simulator::{
            Coordinate,
            Direction::{Down, Left, Right, Up},
            Motion, RopeMap,
        },
    };

    #[test]
    fn given_example_part_parse_check_1() {
        let input_file = File::open("./example.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let expected_motions = vec![
            Motion::new(Right, 4),
            Motion::new(Up, 4),
            Motion::new(Left, 3),
            Motion::new(Down, 1),
            Motion::new(Right, 4),
            Motion::new(Down, 1),
            Motion::new(Left, 5),
            Motion::new(Right, 2),
        ];

        assert_eq!(
            expected_motions, parsed,
            "parser should have parsed motions"
        );
    }

    #[test]
    fn given_example_part_1() {
        let input_file = File::open("./example.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let mut rope_map = RopeMap::init();

        for motion in parsed {
            rope_map.move_rope_n_times(motion.direction, motion.amount);
        }

        let no_unique_tail_positions = rope_map
            .path
            .into_iter()
            .map(|rco| rco.tail)
            .collect::<HashSet<Coordinate>>()
            .iter()
            .count();

        assert_eq!(13, no_unique_tail_positions);
    }
}
