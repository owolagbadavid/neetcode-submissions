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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cache: HashMap<(*const RefCell<TreeNode>, bool), i32> = HashMap::new();
        max(Self::dfs(&root, true, &mut cache), Self::dfs(&root, false, &mut cache))
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, can: bool, cache: &mut HashMap<(*const RefCell<TreeNode>, bool), i32>) -> i32 {
        let key = (Self::node_ptr(root), can);
        if cache.contains_key(&key) {
            return cache[&key];
        }
        if let Some(root) = root {
            let root = root.borrow();
            let left = &root.left;
            let right = &root.right;
            let val = root.val;
            let mut res = 0;
            if can {
                res = val + (Self::dfs(left, false, cache) + Self::dfs(right, false, cache));
            }
            res = max(res, Self::dfs(left, true, cache) + Self::dfs(right, true, cache));
            cache.insert(key, res);
            res
        } else {
            return 0;
        }
    }

    fn node_ptr(node: &Option<Rc<RefCell<TreeNode>>>) -> *const RefCell<TreeNode> {
        match node {
            Some(rc) => Rc::as_ptr(rc),
            None => std::ptr::null(),
        }
    }
}
