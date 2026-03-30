# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        int1 = int2 = 0

        cur = l1
        m = 1
        while cur:
            int1 += m * cur.val
            cur = cur.next
            m *= 10
        
        cur = l2
        m = 1
        while cur:
            int2 += m * cur.val
            cur = cur.next
            m *= 10

        num = int1 + int2
        d = 1

        dummy = cur = ListNode()
        cur.next = ListNode(0)
        while num:
            n = num%(d*10)
            cur.next = ListNode(n//d)
            cur = cur.next
            num -= n
            d *= 10

        return dummy.next