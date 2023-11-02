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
            .iter()
            .count();

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
            .iter()
            .count();

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

        let mut third_knot_path: Vec<Coordinate> = Vec::new();
        for knotted_rope_co in knotted_rope_map.path.iter() {
            let tail = &knotted_rope_co.knots[2].head;
            third_knot_path.push(tail.clone());
        }

        assert_eq!(third_knot_path[0].x, 0, "0 move x should be 0");
        assert_eq!(third_knot_path[1].x, 0, "1 move x should be 0");
        assert_eq!(third_knot_path[2].x, 0, "2 move x should be 0");
        assert_eq!(third_knot_path[3].x, 1, "3 move x should be 1");
        assert_eq!(third_knot_path[4].x, 2, "4 move x should be 2");
        assert_eq!(third_knot_path[5].x, 2, "5 move x shoud be 2");
        assert_eq!(third_knot_path[6].x, 3, "6 move x shoud be 3");

        assert_eq!(third_knot_path[0].y, 0, "0 move y should be 0");
        assert_eq!(third_knot_path[1].y, 0, "1 move y should be 0");
        assert_eq!(third_knot_path[2].y, 0, "2 move y should be 0");
        assert_eq!(third_knot_path[3].y, 0, "3 move y should be 0");
        assert_eq!(third_knot_path[4].y, 0, "4 move y should be 0");
        assert_eq!(third_knot_path[5].y, 0, "5 move y shoud be 0");
        assert_eq!(third_knot_path[6].y, 1, "6 move y shoud be 1");
    }

    #[test]
    fn given_example_2_part_2() {
        let input_file = File::open("./example2.txt").expect("opening file");
        let parsed = input_parser::parse_input(&input_file).expect("parsing file");

        let mut knotted_rope_map = KnottedRopeMap::init(9);

        let mut motions_iter = parsed.into_iter();

        let motion = motions_iter.next().unwrap();
        knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);

        assert_eq!(Right, motion.direction);
        assert_eq!(5, motion.amount);
        assert_eq!(0, knotted_rope_map.get_last_co().get_tail().tail.x);
        assert_eq!(0, knotted_rope_map.get_last_co().get_tail().tail.y);

        let motion = motions_iter.next().unwrap();
        knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);

        assert_eq!(Up, motion.direction);
        assert_eq!(8, motion.amount);
        assert_eq!(0, knotted_rope_map.get_last_co().get_tail().tail.x);
        assert_eq!(0, knotted_rope_map.get_last_co().get_tail().tail.y);

        let motion = motions_iter.next().unwrap();
        knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);

        assert_eq!(Left, motion.direction);
        assert_eq!(8, motion.amount);
        assert_eq!(1, knotted_rope_map.get_last_co().get_tail().tail.x);
        assert_eq!(3, knotted_rope_map.get_last_co().get_tail().tail.y);
        // for motion in parsed {
        // }

        let no_unique_tail_positions = knotted_rope_map
            .path
            .iter()
            .map(|rco| rco.get_tail().tail.clone())
            .collect::<HashSet<Coordinate>>()
            .iter()
            .count();

        assert_eq!(36, no_unique_tail_positions);
    }

    // #[test]
    // fn given_example_part_2_chunk() {
    //     let input_file = File::open("./example2.txt").expect("opening file");
    //     let parsed = input_parser::parse_input(&input_file).expect("parsing file");
    //
    //     let mut knotted_rope_map = KnottedRopeMap::init(10);
    //
    //     for motion in parsed[0..2].into_iter() {
    //         knotted_rope_map.move_rope_n_times(motion.direction.clone(), motion.amount);
    //     }
    //
    //     // println!(
    //     //     "{:?}",
    //     //     knotted_rope_map
    //     //         .path
    //     //         .iter()
    //     //         .map(|kr| kr.get_tail())
    //     //         .collect::<Vec<_>>()
    //     // );
    //
    //     let mut hash_set: HashSet<Coordinate> = HashSet::new();
    //     for (i, knotted_rope_co) in knotted_rope_map.path.iter().enumerate() {
    //         let tail = &knotted_rope_co.knots[4].head;
    //
    //         let new = hash_set.insert(tail.clone());
    //         println!("{} {:?}", i, tail);
    //     }
    //
    //     let no_unique_tail_positions = hash_set.iter().count();
    //
    //     assert_eq!(0, no_unique_tail_positions);
    // }
}
