# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head or not head.next:
            return
        l1 = slow = head
        fast = head.next
        
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        cur = slow.next
        l2 = slow.next = None
        while cur:
            tmp = cur.next
            cur.next = l2
            l2 = cur
            cur = tmp
        
        cur = dummy = ListNode()
        i = 1
        while l1 or l2:
            if i % 2 and l1:
                cur.next = l1
                l1 = l1.next
            else:
                cur.next = l2
                l2 = l2.next
            i += 1
            cur = cur.next

