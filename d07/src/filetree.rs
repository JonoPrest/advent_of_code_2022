use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct FileTreeNode(Rc<RefCell<Node>>);

#[derive(Debug)]
enum Parent {
    Root,
    Node(FileTreeNode),
}

#[derive(Debug)]
pub struct Node {
    parent: Parent,
    name: String,
    item: Item,
}

#[derive(Debug)]
pub enum Item {
    File {
        size: i32,
    },
    Dir {
        content: HashMap<String, FileTreeNode>,
    },
}

impl Item {
    fn new_file(size: i32) -> Self {
        Self::File { size }
    }
    fn new_dir() -> Self {
        Self::Dir {
            content: HashMap::new(),
        }
    }
}

impl Node {
    pub fn create_root(name: String) -> Self {
        Node {
            parent: Parent::Root,
            name,
            item: Item::new_dir(),
        }
    }

    fn new(name: String, item: Item, parent: FileTreeNode) -> Self {
        Node {
            parent: Parent::Node(parent),
            name,
            item,
        }
    }

    fn new_dir(name: String, parent: FileTreeNode) -> Self {
        Self::new(name, Item::new_dir(), parent)
    }

    fn new_file(name: String, file_size: i32, parent: FileTreeNode) -> Self {
        Self::new(name, Item::new_file(file_size), parent)
    }

    // pub fn mkdir(&self, dir_name: String) -> anyhow::Result<()> {
    //     let node_ref = Rc::new(RefCell::new(self));
    //     match &mut node_ref.borrow_mut().item {
    //         Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
    //         Item::Dir { content } => {
    //             content.entry(dir_name.clone()).or_insert_with(|| {
    //                 Rc::new(RefCell::new(Self::new_dir(dir_name, node_ref.clone())))
    //             });
    //         }
    //     }
    //     Ok(())
    // }
    //
    // pub fn touch(self, file_name: String, file_size: i32) -> anyhow::Result<()> {
    //     let mutable_self = Rc::new(RefCell::new(self));
    //     match &mut mutable_self.borrow_mut().item {
    //         Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
    //         Item::Dir { content } => {
    //             content.entry(file_name.clone()).or_insert_with(|| {
    //                 Rc::new(RefCell::new(Self::new_file(
    //                     file_name,
    //                     file_size,
    //                     mutable_self.clone(),
    //                 )))
    //             });
    //         }
    //     }
    //     Ok(())
    // }
    //
    // pub fn cd(&self, dir_name: String) -> anyhow::Result<FileTreeNode> {
    //     match &self.item {
    //         Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
    //         Item::Dir { content } => {
    //             let node = content
    //                 .get(&dir_name)
    //                 .ok_or_else(|| anyhow!("No dir named {}", dir_name))?;
    //             Ok(node.clone())
    //         }
    //     }
    // }
    //
    // pub fn cd_up(&self) -> anyhow::Result<FileTreeNode> {
    //     match &self.parent {
    //         Parent::Root => Err(anyhow!("Cannot cd .., {} is the root", self.name))?,
    //         Parent::Node(parent) => Ok(parent.clone()),
    //     }
    // }
    // pub fn mkdir(&mut self, dir_name: String) -> anyhow::Result<()> {
    //     match &mut self.item {
    //         Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
    //         Item::Dir { content } => {
    //             let this = self;
    //             content
    //                 .entry(dir_name.clone())
    //                 .or_insert_with(|| Self::new(dir_name, Box::new(this)))
    //         }
    //     };
    //     Ok(())
    // }
    // pub fn get_item_at_path(&self, path: PathBuf) -> anyhow::Result<Option<&Node>> {
    //     let mut components: VecDeque<_> = path.iter().collect();
    //     let first_item_opt = components.pop_front();
    //     match first_item_opt {
    //         None => Ok(Some(self)),
    //         Some(first_item_os_str) => {
    //             let first_item = first_item_os_str
    //                 .to_str()
    //                 .ok_or_else(|| anyhow!("failed converting OsString"))?;
    //
    //             match &self.item {
    //                 Item::File { size: _ } => {
    //                     Err(anyhow!("Expected dir instead of file at {}", first_item))
    //                 }
    //                 Item::Dir { content } => {
    //                     let node_opt = content.get(first_item);
    //
    //                     match node_opt {
    //                         None => Ok(None),
    //                         Some(node) => {
    //                             let next_path = PathBuf::from_iter(components.iter());
    //
    //                             Self::get_item_at_path(node, next_path)
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // pub fn create_dir_at_path(&mut self, path: PathBuf) -> anyhow::Result<()> {
    //     let mut components: VecDeque<_> = path.iter().collect();
    //     let first_item_opt = components.pop_front();
    //     match first_item_opt {
    //         None => Ok(()),
    //         Some(first_item_os_str) => {
    //             let first_item = first_item_os_str
    //                 .to_str()
    //                 .ok_or_else(|| anyhow!("failed converting OsString"))?
    //                 .to_string();
    //
    //             match &mut self.item {
    //                 Item::File { size: _ } => {
    //                     Err(anyhow!("Expected dir instead of file at {}", first_item))
    //                 }
    //                 Item::Dir { content } => {
    //                     let next_node = content.entry(first_item.clone()).or_insert(Node {
    //                         name: first_item,
    //                         item: Item::Dir {
    //                             content: HashMap::new(),
    //                         },
    //                     });
    //
    //                     let next_path = PathBuf::from_iter(components.iter());
    //                     Self::create_dir_at_path(next_node, next_path)
    //                 }
    //             }
    //         }
    //     }
    // }
}

impl FileTreeNode {
    pub fn create_root(name: String) -> Self {
        FileTreeNode(Rc::new(RefCell::new(Node::create_root(name))))
    }

    fn new(node: Node) -> Self {
        FileTreeNode(Rc::new(RefCell::new(node)))
    }

    fn new_dir(name: String, parent: Self) -> Self {
        let node = Node::new_dir(name, parent);
        Self::new(node)
    }

    fn new_file(name: String, file_size: i32, parent: Self) -> Self {
        let node = Node::new_file(name, file_size, parent);
        Self::new(node)
    }

    pub fn get_name(&self) -> String {
        self.0.borrow_mut().name.clone()
    }

    pub fn mkdir(&self, dir_name: String) -> anyhow::Result<()> {
        match &mut self.0.borrow_mut().item {
            Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
            Item::Dir { content } => {
                content
                    .entry(dir_name.clone())
                    .or_insert_with(|| Self::new_dir(dir_name, self.clone()));
            }
        }
        Ok(())
    }

    pub fn touch(&self, file_name: String, file_size: i32) -> anyhow::Result<()> {
        match &mut self.0.borrow_mut().item {
            Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
            Item::Dir { content } => {
                content
                    .entry(file_name.clone())
                    .or_insert_with(|| Self::new_file(file_name, file_size, self.clone()));
            }
        }
        Ok(())
    }

    pub fn cd(&self, dir_name: String) -> anyhow::Result<FileTreeNode> {
        match &self.0.borrow_mut().item {
            Item::File { .. } => Err(anyhow!("Expected node to be dir"))?,
            Item::Dir { content } => {
                let node = content
                    .get(&dir_name)
                    .ok_or_else(|| anyhow!("No dir named {}", dir_name))?;
                Ok(node.clone())
            }
        }
    }

    pub fn cd_up(&self) -> anyhow::Result<FileTreeNode> {
        match &self.0.borrow_mut().parent {
            Parent::Root => Err(anyhow!(
                "Cannot cd .., {} is the root",
                self.clone().get_name()
            ))?,
            Parent::Node(parent) => Ok(parent.clone()),
        }
    }

    pub fn get_size(&self) -> i32 {
        match &self.0.borrow_mut().item {
            Item::File { size, .. } => *size,
            Item::Dir { content } => content.values().map(|ft| ft.get_size()).sum(),
        }
    }

    fn is_dir(&self) -> bool {
        match self.0.borrow_mut().item {
            Item::Dir { .. } => true,
            Item::File { .. } => false,
        }
    }

    pub fn sizes_at_most_100_000_with_double_count(&self) -> i32 {
        match &self.0.borrow_mut().item {
            Item::File { size, .. } => *size,
            Item::Dir { content } => content
                .values()
                .filter_map(|ft| {
                    if !ft.is_dir() {
                        return None;
                    }
                    let size = ft.get_size();
                    if size <= 100_000 {
                        Some(size + ft.sizes_at_most_100_000_with_double_count())
                    } else {
                        Some(ft.sizes_at_most_100_000_with_double_count())
                    }
                })
                .sum(),
        }
    }

    fn find_all_dirs_of_size_gt_and_push(&self, size: i32, dirs: &mut Vec<Self>) {
        if self.is_dir() && self.get_size() > size {
            dirs.push(self.clone());
        }

        match &self.0.borrow_mut().item {
            Item::File { .. } => (), //Do nothing
            Item::Dir { content } => {
                for ft in content.values() {
                    ft.find_all_dirs_of_size_gt_and_push(size, dirs);
                }
            }
        }
    }

    pub fn find_all_dirs_of_size_gt(&self, size: i32) -> Vec<Self> {
        let mut dirs = Vec::new();
        self.find_all_dirs_of_size_gt_and_push(size, &mut dirs);

        dirs
    }

    pub fn find_smallest_dir_of_min_size(&self, size: i32) -> Option<Self> {
        self.find_all_dirs_of_size_gt(size)
            .into_iter()
            .min_by(|x, y| x.get_size().cmp(&y.get_size()))
    }

    pub fn get_remaining_space(&self, total_space: i32) -> i32 {
        total_space - self.get_size()
    }
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn check_path_iter() {
        let path = PathBuf::from_str("/first/second/third/").unwrap();

        let expected: Vec<_> = vec!["/", "first", "second", "third"];
        let actual: Vec<_> = path.iter().map(|item| item.to_str().unwrap()).collect();
        assert_eq!(expected, actual);
    }
}
