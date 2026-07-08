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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None=> None,
            Some(root_rc) => {
                {
                    let mut node = root_rc.borrow_mut();
                    if key > node.val {
                        let right = node.right.take();
                        node.right = Self::delete_node(right, key)
                    } else if key < node.val {
                        let left = node.left.take();
                        node.left = Self::delete_node(left, key)
                    } else {
                        if node.left.is_none() {
                            return node.right.take();
                        } else if node.right.is_none() {
                            return node.left.take();
                        } else {
                            let right = node.right.take();
                            let min = Self::find_min(right.as_ref().unwrap().clone());
                            node.right = Self::delete_node(right, min);
                            node.val = min;
                        }
                    }
                }
                Some(root_rc)
            }
        }
    }

    fn find_min(root: Rc<RefCell<TreeNode>>) -> i32 {
        let mut cur = root;
        while cur.borrow().left.is_some() {
            let left = cur.borrow().left.as_ref().unwrap().clone();
            cur = left;
        }
        cur.borrow().val
    }
}
