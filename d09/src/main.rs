use anyhow::Context;
use std::{collections::HashSet, fs::File};

mod input_parser;
mod rope_simulator;

use rope_simulator::{Coordinate, RopeMap};

use crate::rope_simulator::KnottedRopeMap;

fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;

    // println!("parsed: {:?}", parsed);

    let mut rope_map = RopeMap::init();

    for motion in parsed.iter() {
        rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);
    }

    let no_unique_tail_positions = rope_map
        .path
        .into_iter()
        .map(|rco| rco.tail)
        .collect::<HashSet<Coordinate>>()
        .len();

    println!("No unique tail positions:");
    println!("{}", no_unique_tail_positions);

    //Part 2
    let mut knotted_rope_map = KnottedRopeMap::init(9);
    for motion in parsed {
        knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);
    }

    let no_unique_tail_positions = knotted_rope_map
        .path
        .iter()
        .map(|rco| rco.get_tail().tail.clone())
        .collect::<HashSet<Coordinate>>()
        .len();

    println!("No unique tail positions on knotted rope:");
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
            KnottedRopeMap, Motion, RopeMap,
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
            .len();

        assert_eq!(13, no_unique_tail_positions);
    }

    #[test]
    fn validated_correct_part_1_answer() {
        let input_file = File::open("./my_input.txt").expect("opening file");
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
            .len();

        assert_eq!(6563, no_unique_tail_positions);
    }

    #[test]
    fn given_example_1_part_2() {
        let input_file = File::open("./example.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let mut knotted_rope_map = KnottedRopeMap::init(10);

        for motion in parsed {
            knotted_rope_map.move_rope_n_times(motion.direction, motion.amount);
        }

        let third_knot_path: Vec<Coordinate> = knotted_rope_map
            .path
            .iter()
            .map(|knotted_rope_co| knotted_rope_co.knots[2].head.clone())
            .collect();

        assert_eq!(third_knot_path[0].x, 0, "0 move x should be 0");
        assert_eq!(third_knot_path[2].x, 0, "2 move x should be 0");
        assert_eq!(third_knot_path[3].x, 1, "3 move x should be 1");
        assert_eq!(third_knot_path[4].x, 2, "4 move x should be 2");
        assert_eq!(third_knot_path[5].x, 2, "5 move x shoud be 2");
        assert_eq!(third_knot_path[6].x, 3, "6 move x shoud be 3");

        assert_eq!(third_knot_path[0].y, 0, "0 move y should be 0");
        assert_eq!(third_knot_path[5].y, 0, "5 move y shoud be 0");
        assert_eq!(third_knot_path[6].y, 1, "6 move y shoud be 1");

        let fourth_knot_path: Vec<Coordinate> = knotted_rope_map
            .path
            .iter()
            .map(|knotted_rope_co| knotted_rope_co.knots[3].head.clone())
            .collect();

        assert_eq!(fourth_knot_path[0].x, 0, "0 move x should be 0");
        assert_eq!(fourth_knot_path[3].x, 0, "3 move x should be 0");
        assert_eq!(fourth_knot_path[4].x, 1, "4 move x should be 1");
        assert_eq!(fourth_knot_path[5].x, 1, "5 move x shoud be 1");
        assert_eq!(fourth_knot_path[6].x, 2, "6 move x shoud be 2");

        assert_eq!(fourth_knot_path[0].y, 0, "0 move y should be 0");
        assert_eq!(fourth_knot_path[5].y, 0, "5 move y shoud be 0");
        assert_eq!(fourth_knot_path[6].y, 1, "6 move y shoud be 1");
    }

    #[test]
    fn given_example_2_part_2_main() {
        let input_file = File::open("./example2.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let mut knotted_rope_map = KnottedRopeMap::init(9);

        for motion in parsed {
            knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);
        }

        let no_unique_tail_positions = knotted_rope_map
            .path
            .iter()
            .map(|rco| rco.get_tail().tail.clone())
            .collect::<HashSet<Coordinate>>()
            .len();

        assert_eq!(36, no_unique_tail_positions);
    }

    #[test]
    fn given_example_2_part_2() {
        let input_file = File::open("./example2.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let mut knotted_rope_map = KnottedRopeMap::init(9);

        for motion in parsed {
            knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);
        }

        let no_unique_tail_positions = knotted_rope_map
            .path
            .iter()
            .map(|rco| rco.get_tail().tail.clone())
            .collect::<HashSet<Coordinate>>()
            .len();

        assert_eq!(36, no_unique_tail_positions);
    }
}
