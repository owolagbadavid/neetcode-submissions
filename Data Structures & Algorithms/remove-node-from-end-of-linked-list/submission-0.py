# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        cur = head
        prev = None
        while cur:
            tmp = cur.next
            cur.next = prev
            prev = cur
            cur = tmp

        cur = dummy = ListNode()
        dummy.next = prev
        end = dummy.next.next
        i = 1
        while i < n:
            cur = cur.next
            end = cur.next.next
            i += 1
        cur.next = end

        cur = dummy.next
        prev = None
        while cur:
            tmp = cur.next
            cur.next = prev
            prev = cur
            cur = tmp
        return prev
