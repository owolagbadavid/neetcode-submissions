// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut node = root.borrow_mut();
                let left = node.left.take();
                let right = node.right.take();
                node.left = Self::remove_leaf_nodes(left, target);
                node.right = Self::remove_leaf_nodes(right, target);
                if node.left.is_none() && node.right.is_none() && node.val == target {
                    return None;
                }
                drop(node);
                Some(root)
            }
        }
    }
}
