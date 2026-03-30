# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        leng = 0 
        cur = head
        dummy = ListNode()
        while cur:
            cur = cur.next
            leng += 1
        times = leng // k
        first = True

        cur = head
        prev = None
        while times > 0:
            i = 1
            oldHead = cur
            while i <= k:
                tmp = cur.next
                cur.next = prev
                prev = cur
                cur = tmp
                if i == k and first:
                    dummy.next = prev
                    first = False
                i += 1
            times -= 1
            
            newHead = cur
            if times:
                for i in range(k-1):
                    newHead = newHead.next
            oldHead.next = newHead
        return dummy.next