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
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let node = TreeNode::new(val);
        Self::dfs(root, node)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None =>  Some(Rc::new(RefCell::new(node))),
            Some(root_rc) => {
               {
                    let mut root_val = root_rc.borrow_mut();
                    if node.val > root_val.val {
                        let right = root_val.right.take();
                        root_val.right = Self::dfs(right, node);
                    } else {
                        let left = root_val.left.take();
                        root_val.left = Self::dfs(left, node);
                    }
               }
               Some(root_rc)
            }
        }
    }
}
