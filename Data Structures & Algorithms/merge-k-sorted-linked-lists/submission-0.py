# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:    
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        cur = dummy = ListNode()
        i = 0
        while i < len(lists):
            if cur.next:
                l1 = cur.next
                l2 = lists[i]
            else:
                l1 = lists[i] 
                l2 = lists[i+1] if i < len(lists)-1 else None
                i += 1
            while l1 or l2:
                val1 = l1.val if l1 else float('infinity')
                val2 = l2.val if l2 else float('infinity')
                if val1 < val2:
                    cur.next = l1
                    l1 = l1.next
                else:
                    cur.next = l2
                    l2 = l2.next
                cur = cur.next
            cur = dummy
            i += 1

        return dummy.next
