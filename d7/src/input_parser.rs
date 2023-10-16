use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub enum LogLine {
    Command(ShellCommand),
    OutputVal(FileDir),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileDir {
    File { name: String, size: i32 },
    Dir { name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ShellCommand {
    Cd(CdVal),
    Ls,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CdVal {
    UpDir,
    InToDir(String),
}

type LogLines = Vec<LogLine>;

type LineComponents<'a> = Vec<&'a str>;

pub fn get_shell_command_from_line_components(
    components: LineComponents,
) -> anyhow::Result<ShellCommand> {
    let command = match components
        .get(1)
        .ok_or_else(|| anyhow!("no 1 index in line components"))?
        .as_ref()
    {
        "cd" => {
            let cd_val = match components
                .get(2)
                .ok_or_else(|| anyhow!("Expected val for cd command"))?
                .as_ref()
            {
                ".." => CdVal::UpDir,
                dir_name => CdVal::InToDir(dir_name.to_string()),
            };

            ShellCommand::Cd(cd_val)
        }
        "ls" => ShellCommand::Ls,
        unknown_cmd => Err(anyhow!("Unexpected command {}", unknown_cmd))?,
    };
    Ok(command)
}

pub fn get_file_dir_from_line_components(components: LineComponents) -> anyhow::Result<FileDir> {
    let name = components
        .get(1)
        .ok_or_else(|| anyhow!("Expected val for log"))?
        .to_string();
    let command = match components
        .first()
        .ok_or_else(|| anyhow!("no 1 index in line components"))?
        .as_ref()
    {
        "dir" => FileDir::Dir { name },
        size_str => {
            let size: i32 = size_str.parse().context("parsing size from str")?;

            FileDir::File { name, size }
        }
    };
    Ok(command)
}

pub fn parse_input(file: &File) -> anyhow::Result<LogLines> {
    let reader = BufReader::new(file);

    let parsed = reader
        .lines()
        .map(|line_res| {
            let line = line_res.context("getting line from reader")?;
            let components: Vec<&str> = line.split(' ').collect();

            let is_shell_command = components
                .first()
                .ok_or_else(|| anyhow!("no 0 index in line components"))?
                == &"$";

            let log_line = if is_shell_command {
                let command = get_shell_command_from_line_components(components)
                    .context("parsing shell command")?;
                LogLine::Command(command)
            } else {
                let file_dir =
                    get_file_dir_from_line_components(components).context("parsing file_dir")?;
                LogLine::OutputVal(file_dir)
            };

            Ok(log_line)
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("parsing input file")?;

    Ok(parsed)
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use crate::input_parser::{get_file_dir_from_line_components, CdVal, FileDir, ShellCommand};

    use super::{get_shell_command_from_line_components, parse_input};

    #[test]
    fn check_shell_command_parse() {
        let command = "$ cd ..";
        let parsed = get_shell_command_from_line_components(command.split(' ').collect()).unwrap();

        assert_eq!(ShellCommand::Cd(CdVal::UpDir), parsed);
    }
    #[test]
    fn check_shell_command_parse_2() {
        let command = "$ cd test";
        let parsed = get_shell_command_from_line_components(command.split(' ').collect()).unwrap();

        assert_eq!(ShellCommand::Cd(CdVal::InToDir("test".to_string())), parsed);
    }
    #[test]
    fn check_shell_command_parse_3() {
        let command = "$ ls";
        let parsed = get_shell_command_from_line_components(command.split(' ').collect()).unwrap();

        assert_eq!(ShellCommand::Ls, parsed);
    }

    #[test]
    fn check_log_parse_1() {
        let command = "139569 fhjlbrmp.phd";
        let parsed = get_file_dir_from_line_components(command.split(' ').collect()).unwrap();

        assert_eq!(
            FileDir::File {
                size: 139569,
                name: "fhjlbrmp.phd".to_string()
            },
            parsed
        );
    }
    #[test]
    fn check_log_parse_2() {
        let command = "dir ctctt";
        let parsed = get_file_dir_from_line_components(command.split(' ').collect()).unwrap();

        assert_eq!(
            FileDir::Dir {
                name: "ctctt".to_string()
            },
            parsed
        );
    }

    #[test]
    fn full_file_parse_check() {
        let input_file = File::open("./my_input.txt").expect("opening file");
        parse_input(&input_file).unwrap();
    }
}
