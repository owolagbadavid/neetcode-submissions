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
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        let mut cur = &mut head;
        while cur.as_ref().and_then(|n| n.next.as_ref()).is_some() {
            let pair = cur.as_ref().and_then(|n| n.next.as_ref().map(|m| (n.val, m.val))).unwrap();
            let num = gcd(pair.0, pair.1);
            let mut new = ListNode::new(num);
            let next = cur.as_mut().unwrap().next.take();
            new.next = next;
            cur.as_mut().unwrap().next = Some(Box::new(new));
            cur = &mut cur.as_mut().unwrap().next;
            cur = &mut cur.as_mut().unwrap().next;
        }

        head
    }
}
