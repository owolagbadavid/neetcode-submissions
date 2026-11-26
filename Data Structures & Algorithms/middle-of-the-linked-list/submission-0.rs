// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let head = head.unwrap();
        let mut slow = Some(head.clone());
        let mut fast = Some(head.clone());

        while let Some(fast_box) = fast && fast_box.next.is_some() {
            slow = slow.unwrap().next;
            fast = fast_box.next.unwrap().next;
        }

        slow
    }
}
