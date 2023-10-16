use anyhow::{anyhow, Context};
use input_parser::DataStreamBuffer;
use std::fs::File;

mod input_parser;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let parsed = input_parser::parse_input(&input_file).context("parsing file")?;
    // println!("parsed {:?}", parsed);

    let marker = find_marker(parsed.clone(), 4)?;

    println!("Marker: {}", marker);

    //Part 2

    let marker_2 = find_marker(parsed, 14)?;

    println!("Marker 2: {}", marker_2);
    Ok(())
}

fn check_contains_duplicates(char_slice: &[char]) -> bool {
    char_slice.iter().enumerate().any(|(i, &c)| {
        char_slice
            .iter()
            .enumerate()
            .find(|(i_inner, &c_inner)| &i != i_inner && c == c_inner)
            .is_some()
    })
}

fn find_marker(data_stream: DataStreamBuffer, repeat_size: usize) -> anyhow::Result<usize> {
    Ok(data_stream
        .iter()
        .enumerate()
        .find_map(|(i, _x)| {
            let limited_upper_size = std::cmp::min(i + repeat_size, data_stream.len());
            let slice = &data_stream[i..limited_upper_size];
            let contains_duplicates = check_contains_duplicates(slice);
            if contains_duplicates {
                None
            } else {
                Some(i + repeat_size)
            }
        })
        .ok_or_else(|| anyhow!("No marker found"))?)
}

#[cfg(test)]
mod test {
    use crate::check_contains_duplicates;

    #[test]
    fn test_duplicate_checker() {
        let slice = &['a', 'a', 'b', 'c'];

        assert!(
            check_contains_duplicates(slice),
            "should pick up on duplicate a"
        );
    }
}
