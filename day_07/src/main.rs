use common::utils;
use std::borrow::BorrowMut;
use std::mem::ManuallyDrop;
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
                println!("Made new node of type {:?} with size {:?}", child.item, *child.size.borrow());
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
                println!("Directory {} is now of size {}", self.name, *self.size.borrow());
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

    fn get_subdirectory_size(&self) -> Vec<u32> {
        let mut directory_size: u32 = 0;
        let mut subdirectories : Vec<u32> = Vec::new();
        
        for child in self.children.borrow().values() {
            let mut size: u32 = 0;
            match child.item {
                ItemType::File => {size += *child.size.borrow()},
                ItemType::Dir => {
                    let mut sub = child.get_subdirectory_size();
                    size += sub.iter().sum::<u32>();
                    subdirectories.append(&mut sub);
                },
            };
            directory_size += size;
        }

        subdirectories.push(directory_size);
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
        println!("File: {}", line);
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

    let arr = root.get_subdirectory_size();
    let tot = arr.into_iter().filter(|x| x <= &100_000_u32).sum::<u32>();


    println!("{}", tot);
}
