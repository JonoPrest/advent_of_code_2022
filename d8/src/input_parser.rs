use anyhow::{anyhow, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type TreeHeight = u8;
type TreeRow = Vec<TreeHeight>;
type TreeRows = Vec<TreeRow>;

#[derive(Debug)]
pub struct TreeGrid {
    grid: TreeRows,
}

pub struct TreeGridIterator<'a> {
    tree_grid: &'a TreeGrid,
    x: usize,
    y: usize,
}

pub struct Tree<'a> {
    tree_grid: &'a TreeGrid,
    height: &'a TreeHeight,
    x: usize,
    y: usize,
}

impl<'a> Tree<'a> {
    pub fn is_visible(&self) -> Result<bool> {
        self.tree_grid.check_tree_is_visible(self.x, self.y)
    }
}

impl<'a> Iterator for TreeGridIterator<'a> {
    type Item = Tree<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.tree_grid.get_tree_height(self.x, self.y) {
            Ok(height) => {
                let tree = Tree {
                    tree_grid: self.tree_grid,
                    height,
                    x: self.x,
                    y: self.y,
                };
                self.x += 1;
                Some(tree)
            }
            Err(_) => {
                self.x = 0;
                self.y += 1;
                match self.tree_grid.get_tree_height(self.x, self.y) {
                    Ok(height) => {
                        let tree = Tree {
                            tree_grid: self.tree_grid,
                            height,
                            x: self.x,
                            y: self.y,
                        };

                        self.x += 1;
                        Some(tree)
                    }
                    Err(_) => None,
                }
            }
        }
    }
}

impl TreeGrid {
    pub fn iter(&self) -> TreeGridIterator {
        TreeGridIterator {
            tree_grid: self,
            x: 0,
            y: 0,
        }
    }

    fn get_tree_height(&self, x: usize, y: usize) -> Result<&TreeHeight> {
        Ok(self
            .grid
            .get(y)
            .ok_or_else(|| anyhow!("No row at index {}", y))?
            .get(x)
            .ok_or_else(|| anyhow!("No tree in row at index {}", x))?)
    }

    fn get_tree_row(&self, y: usize) -> Result<&TreeRow> {
        Ok(self
            .grid
            .get(y)
            .ok_or_else(|| anyhow!("No row at index {}", y))?)
    }

    fn get_tree_col_as_row(&self, x: usize) -> Result<TreeRow> {
        Ok(self
            .grid
            .iter()
            .map(|row| {
                row.get(x)
                    .map(|val| *val)
                    .ok_or_else(|| anyhow!("No tree in row at index {}", x))
            })
            .collect::<Result<Vec<_>>>()?)
    }

    fn check_tree_is_visible_in_row(row: &TreeRow, current_tree: &TreeHeight, pos: usize) -> bool {
        let is_visible_before = row[..pos].iter().all(|tree| tree < current_tree);

        if is_visible_before {
            return true;
        }

        let is_visible_after = row[pos + 1..].iter().all(|tree| tree < current_tree);

        if is_visible_after {
            return true;
        }

        false
    }

    fn check_tree_is_visible_x(&self, x: usize, y: usize) -> Result<bool> {
        let current_tree = self.get_tree_height(x, y).context("getting current tree")?;

        let current_row = self.get_tree_row(y).context("getting current tree row")?;

        Ok(Self::check_tree_is_visible_in_row(
            current_row,
            current_tree,
            x,
        ))
    }

    fn check_tree_is_visible_y(&self, x: usize, y: usize) -> Result<bool> {
        let current_tree = self.get_tree_height(x, y).context("getting current tree")?;

        let current_col = self
            .get_tree_col_as_row(x)
            .context("getting current tree col")?;

        Ok(Self::check_tree_is_visible_in_row(
            &current_col,
            current_tree,
            y,
        ))
    }

    fn check_tree_is_outside(&self, x: usize, y: usize) -> Result<bool> {
        Ok(x == 0
            || y == 0
            || y == self.grid.len() - 1
            || x == self.get_tree_row(y).context("getting row")?.len() - 1)
    }

    pub fn check_tree_is_visible(&self, x: usize, y: usize) -> Result<bool> {
        Ok(self
            .check_tree_is_outside(x, y)
            .context("checking outside")?
            || self.check_tree_is_visible_x(x, y).context("checking x")?
            || self.check_tree_is_visible_y(x, y).context("checking y")?)
    }

    pub fn get_grid(&self) -> &TreeRows {
        &self.grid
    }
}

pub fn parse_input(file: &File) -> anyhow::Result<TreeGrid> {
    let reader = BufReader::new(file);
    let tree_rows: TreeRows = reader
        .lines()
        .map(|line| {
            let content = line?;
            content
                .chars()
                .map(|num_str| {
                    num_str
                        .to_digit(10)
                        .map(|num| num as u8)
                        .ok_or_else(|| anyhow!("Couldn't parsed char as digit {}", num_str))
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("parsing tree rows from reader")?;

    Ok(TreeGrid { grid: tree_rows })
}
