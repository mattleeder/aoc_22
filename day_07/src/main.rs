use common::utils;
use std::rc::{Rc, Weak};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
enum ItemType {
    Dir,
    File,
}

#[derive(Debug)]
struct TreeNode {
    name: String,
    children: RefCell<HashMap<String, Rc<TreeNode>>>,
    parent: Weak<TreeNode>,
    item: ItemType,
    size: RefCell<u32>,
}

impl TreeNode {

    fn new(name: String, item: ItemType, size: u32, parent: Option<&mut Rc<TreeNode>>) -> Rc<TreeNode> {
        match parent {
            Some(node) => {
                let child = Rc::new(TreeNode {
                    name,
                    children: RefCell::new(HashMap::new()),
                    parent: Rc::downgrade(&node),
                    item,
                    size: RefCell::new(size),
                });
                node.add_child(Rc::clone(&child));
                return child

            },
            None => {
                let child = Rc::new(TreeNode {
                    name,
                    children: RefCell::new(HashMap::new()),
                    parent: Weak::new(),
                    item,
                    size: RefCell::new(size),
                });
                return child
            }
        }
    }

    fn add_child(&self, child: Rc<TreeNode>) {
        match child.item {
            ItemType::Dir => {},
            ItemType::File => {
                *self.size.borrow_mut() += *child.size.borrow();
            },
        };
        self.children.borrow_mut().insert(child.name.clone(), Rc::clone(&child));
    }

    fn get_child(&self, child_name: &str)  -> Rc<TreeNode> {
        Rc::clone(self.children.borrow().get(child_name).unwrap())
    }

    fn get_parent(&self) -> Option<Rc<TreeNode>> {
        self.parent.upgrade()
    }

    fn get_used_space(&self) -> u32 {
        let mut used_space: u32 = 0;

        for child in self.children.borrow().values() {
            let mut size: u32 = 0;
            match child.item {
                ItemType::File => {size += *child.size.borrow()},
                ItemType::Dir => {size += child.get_used_space()},
            };
            used_space += size;
        }

        used_space
    }

    fn get_subdirectory_used_space(&self) -> Vec<u32> {
        let mut used_space: u32 = 0;
        let mut subdirectories: Vec<u32> = Vec::new();

        for child in self.children.borrow().values() {
            match child.item {
                ItemType::File => {used_space += *child.size.borrow()},
                ItemType::Dir => {
                    used_space += child.get_used_space();
                    subdirectories.append(&mut child.get_subdirectory_used_space());
                },
            }
        }

        subdirectories.push(used_space);
        subdirectories
    }

}

fn main() {
    let binding = utils::read_file().unwrap();
    let mut contents = binding.lines();
    let _first_line: Vec<&str> = (&mut contents).take(1).collect();
    let root: Rc<TreeNode> = TreeNode::new(String::from("/"), ItemType::Dir, 0, None);
    let mut current: Rc<TreeNode> = Rc::clone(&root);

    for line in contents {
        let info: Vec<&str> = line.split(' ').collect();
        if info[0] == "$"  {
            if info[1] == "ls" {
                continue;
            } else if info[1] == "cd" {
                if info[2] == ".." {
                    // Go to parent
                    current = current.get_parent().unwrap();
                } else { // Go to child
                    current = current.get_child(&info[2][..]);
                }
            }
        } else if info[0] == "dir" {
            // Make child directory
            TreeNode::new(info[1].to_string(), ItemType::Dir, 0, Some(&mut current));
        } else {
            // Make child File
            TreeNode::new(info[1].to_string(), ItemType::File, info[0].parse::<u32>().unwrap(), Some(&mut current));
        }
    }

    let arr = root.get_subdirectory_used_space();
    let p1 = arr.clone().into_iter().filter(|x| x <= &100_000).sum::<u32>();
    let unused_space: u32 = 30_000_000 - (70_000_000 - root.get_used_space());
    let arr = arr.into_iter().filter(|x| x >= &unused_space);
    let p2 = arr.min().unwrap();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
