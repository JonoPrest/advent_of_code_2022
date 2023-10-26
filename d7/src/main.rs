use anyhow::{anyhow, Context};
use input_parser::{CdVal, FileDir, LogLine, ShellCommand};
use std::fs::File;

mod filetree;
mod input_parser;

use filetree::FileTreeNode;
fn main() -> anyhow::Result<()> {
    let input_file = File::open("./my_input.txt").context("opening file")?;

    //Part 1
    let lexical_data = input_parser::parse_input(&input_file).context("parsing file")?;

    let root_node = construct_file_tree(lexical_data).context("main construction")?;

    println!("Total size: {}", root_node.get_size());
    //Part One Answer
    println!(
        "Total of sizes at most 100_000 including double count of files: {}",
        root_node.sizes_at_most_100_000_with_double_count()
    );
    Ok(())
}

fn execute_log_line(log_line: LogLine, current_node: FileTreeNode) -> anyhow::Result<FileTreeNode> {
    match log_line {
        LogLine::Command(cmd) => match cmd {
            ShellCommand::Cd(cd_type) => match cd_type {
                CdVal::InToDir(dir_name) => {
                    let next_node = current_node
                        .cd(dir_name.clone())
                        .context(format!("Cd into {}", dir_name))?;
                    Ok(next_node)
                }
                CdVal::UpDir => {
                    let next_node = current_node
                        .cd_up()
                        .context(format!("Cd up from {}", current_node.get_name()))?;
                    Ok(next_node)
                }
            },
            ShellCommand::Ls => Ok(current_node), //Do nothing, only the the lines following this
                                                  //will action something
        },
        LogLine::OutputVal(val) => match val {
            FileDir::File { size, name } => {
                //Create the file and return the current node
                current_node.touch(name, size)?;
                Ok(current_node)
            }
            FileDir::Dir { name } => {
                //Create the dir and return the current node
                current_node
                    .mkdir(name.clone())
                    .context(format!("MkDir on cd into {}", name))?;
                Ok(current_node)
            }
        },
    }
}

fn execute_log_lines(
    mut log_line_iter: impl Iterator<Item = LogLine>,
    current_node: FileTreeNode,
) -> anyhow::Result<FileTreeNode> {
    match log_line_iter.next() {
        None => Ok(current_node),
        Some(log_line) => {
            let next_node = execute_log_line(log_line, current_node)?;
            execute_log_lines(log_line_iter, next_node)
        }
    }
}

fn construct_file_tree(log_lines: Vec<LogLine>) -> anyhow::Result<FileTreeNode> {
    let mut log_lines_iter = log_lines.into_iter();
    let root_dir_name = log_lines_iter
        .next()
        .ok_or_else(|| anyhow!("No first log line"))
        .and_then(|v| match v {
            LogLine::Command(ShellCommand::Cd(CdVal::InToDir(root_dir_name))) => Ok(root_dir_name),
            _ => Err(anyhow!("First command is not cd")),
        })
        .context("getting root dir name")?;

    let root_node = FileTreeNode::create_root(root_dir_name);
    execute_log_lines(log_lines_iter, root_node.clone())?;

    Ok(root_node)
}

#[cfg(test)]
mod test {
    use anyhow::Context;
    use std::fs::File;

    use crate::input_parser;
    #[test]
    fn day_one_example() -> anyhow::Result<()> {
        let input_file = File::open("./day_1_example.txt").context("opening file")?;
        let lexical_data = input_parser::parse_input(&input_file).context("parsing file")?;

        let root_node = super::construct_file_tree(lexical_data).context("main construction")?;

        let expected_val = 95437;
        assert_eq!(
            expected_val,
            root_node.sizes_at_most_100_000_with_double_count()
        );
        Ok(())
    }
}
