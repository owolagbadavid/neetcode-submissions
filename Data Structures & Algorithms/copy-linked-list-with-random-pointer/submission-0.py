"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        cur = head
        mapp = {}
        cur2 = dummy = Node(0)
        while cur:
            h = n = r = None
            if cur in mapp:
                h = mapp[cur]
            else:
                h = Node(cur.val)
                mapp[cur] = h
            if cur.next in mapp:
                n = mapp[cur.next]
            elif cur.next:
                n = Node(cur.next.val)
                mapp[cur.next] = n
            if cur.random in mapp:
                r = mapp[cur.random]
            elif cur.random:
                r = Node(cur.random.val)
                mapp[cur.random] = r
            h.next = n
            h.random = r
            cur2.next = h
            cur = cur.next
            cur2 = cur2.next
            
        return dummy.next