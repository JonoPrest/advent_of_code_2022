use anyhow::Context;
use std::{fs::File, io::Read};

pub type DataStreamBuffer = Vec<char>;
pub fn parse_input(file: &File) -> anyhow::Result<DataStreamBuffer> {
    let mut file_copy = file.clone();
    let mut contents = String::new();
    file_copy
        .read_to_string(&mut contents)
        .context("reading file to string")?;

    Ok(contents.chars().collect())
}
