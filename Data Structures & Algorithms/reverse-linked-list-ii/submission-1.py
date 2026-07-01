# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
        if left == right:
            return head

        i = 1
        before = start = head
        end = head

        while i <= right:
            end = end.next
            i += 1

        prev = end

        i = 1;
        while i < left:
            before = start
            start = start.next
            i += 1

        while start != end:
            tmp = start.next
            start.next = prev
            prev = start
            start = tmp
        
        if left > 1:
            before.next = prev
        
        return prev if left == 1 else head

        

       