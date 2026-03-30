# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head or not head.next:
            return
        l1 = l2 = fast = head
        
        while fast and fast.next:
            l2 = l2.next
            fast = fast.next.next
        
        cur = l2
        l2 = None
        while cur:
            tmp = cur.next
            cur.next = l2
            l2 = cur
            cur = tmp
        
        cur = dummy = ListNode()
        i = 1
        while l1 and l2:
            if i % 2:
                print(l1.val)
                cur.next = l1
                l1 = l1.next
            else:
                print(l2.val)
                cur.next = l2
                l2 = l2.next
            i += 1
            cur = cur.next

