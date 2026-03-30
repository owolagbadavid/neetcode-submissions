class LRUCache:

    def __init__(self, capacity: int):
        self.cap = capacity
        self.dic = {}
        self.head = ListNode()
        self.tail = ListNode()
        self.head.next = self.tail
        self.tail.prev = self.head

    def get(self, key: int) -> int:
        if key in self.dic:
            node = self.dic[key]
            node.prev.next = node.next
            node.next.prev = node.prev
            tmp = self.tail.prev
            self.tail.prev = node
            node.next = self.tail
            tmp.next = node
            node.prev = tmp
            return node.val[1]
        return -1

    def put(self, key: int, value: int) -> None:
        if len(self.dic) >= self.cap and key not in self.dic:
            node = self.head.next
            oldKey = node.val[0]
            self.dic.pop(oldKey)
            self.head.next = node.next
            node.next.prev = self.head
            node.next = node.prev = None
    
        node = ListNode((key, value))
        if key in self.dic:
            node = self.dic[key]
            node.prev.next = node.next
            node.next.prev = node.prev
            node.val = (key, value)
        
        tmp = self.tail.prev
        self.tail.prev = node
        node.next = self.tail
        tmp.next = node
        node.prev = tmp
        self.dic[key] = node

        
class ListNode:
    def __init__(self, val=None, next=None, prev=None):
        self.val = val
        self.next = next
        self.prev = prev