// Definition for a QuadTree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: bool,
//     pub is_leaf: bool,
//     pub top_left: Option<Rc<RefCell<Node>>>,
//     pub top_right: Option<Rc<RefCell<Node>>>,
//     pub bottom_left: Option<Rc<RefCell<Node>>>,
//     pub bottom_right: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: bool, is_leaf: bool) -> Self {
//         Node {
//             val,
//             is_leaf,
//             top_left: None,
//             top_right: None,
//             bottom_left: None,
//             bottom_right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        return Self::grid(grid.len(), 0, 0, &grid);
    }

    fn grid(n: usize, r: usize, c: usize, grid: &Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        let mut leaf = true;
        let val = grid[r][c];
        for i in r..r+n {
            for j in c..c+n {
                if grid[i][j] != val {
                    leaf = false;
                    break;
                }
            }
        }

        if leaf {
            let val = if val == 1 {true} else {false};
            return Some(Rc::new(RefCell::new(Node::new(val, leaf))));
        }

        let mut root = Node::new(leaf, leaf);
        let n = n / 2;
        root.top_left = Self::grid(n, r, c, grid);
        root.top_right = Self::grid(n, r, c+n, grid);
        root.bottom_left = Self::grid(n, r+n, c, grid);
        root.bottom_right = Self::grid(n, r+n, c+n, grid);

        return Some(Rc::new(RefCell::new(root)));
    }
}
