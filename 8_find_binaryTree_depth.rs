use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            1 + std::cmp::max(
                max_depth(node.left.clone()),
                max_depth(node.right.clone()),
            )
        }
        None => 0,
    }
}

fn main() {
    // Example binary tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    println!("Maximum depth of the tree: {}", max_depth(root));
}
