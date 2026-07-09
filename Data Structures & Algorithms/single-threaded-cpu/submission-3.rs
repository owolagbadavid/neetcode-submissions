use std::{cmp::Reverse, collections::{BinaryHeap}};

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks: Vec<(&Vec<i32>, usize)> = tasks
            .iter().enumerate().map(|(i, v)| (v, i)).collect();
        tasks.sort();

        let mut t = tasks[0].0[0];
        let mut heap = BinaryHeap::from([Reverse((tasks[0].0[1], tasks[0].1))]);
        let mut res = vec![];
        let mut i = 1;
        while heap.peek().is_some() || i < tasks.len() {
            match heap.pop() {
                Some(x) => {
                    t += x.0.0;
                    res.push(x.0.1 as i32);
                    while i < tasks.len() && t >= tasks[i].0[0] {
                        heap.push(Reverse((tasks[i].0[1], tasks[i].1)));
                        i += 1;
                    }
                }
                None => {
                    t = tasks[i].0[0];
                    heap.push(Reverse((tasks[i].0[1], tasks[i].1)));
                    i += 1;
                }
            }
        }
        res
    }
}
