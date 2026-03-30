# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        l, r = list1, list2
        newList = ListNode()
        cur = newList
        while l or r:
            lVal = l.val if l else float('infinity')
            rVal = r.val if r else float('infinity')
            if lVal < rVal:
                cur.next = ListNode(lVal)
                l = l.next
            else:
                cur.next = ListNode(rVal)
                r = r.next
            cur = cur.next
        return newList.next