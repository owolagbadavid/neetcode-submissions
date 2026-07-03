class ListNode:
    def __init__(self, val=0, next=None, prev=None):
        self.val = val
        self.next = next
        self.prev = prev

class LinkedList:
    def __init__(self):
        self.left = ListNode()
        self.right = ListNode(0, None, self.left)
        self.left.next = self.right
        self.map = {}

    def length(self):
        return len(self.map)

    def pushRight(self, val):
        node = ListNode(val, self.right, self.right.prev)
        self.map[val] = node
        self.right.prev = node
        node.prev.next = node
    
    def pop(self, val):
        if val in self.map:
            node = self.map[val]
            node.prev.next = node.next
            node.next.prev = node.prev
            self.map.pop(val, None)

    def popLeft(self):
        res = self.left.next.val
        self.pop(res)
        return res

    def update(self, val):
        self.pop(val)
        self.pushRight(val)

class LFUCache:

    def __init__(self, capacity: int):
        self.cap = capacity
        self.lfu_count = 0
        self.vals = {}
        self.count = defaultdict(int)
        self.cache = defaultdict(LinkedList)

    def counter(self, key):
        count = self.count[key]
        self.count[key] += 1
        self.cache[count].pop(key)
        self.cache[count+1].pushRight(key)

        if count == self.lfu_count and self.cache[count].length() == 0:
            self.lfu_count += 1

    def get(self, key: int) -> int:
        if key not in self.vals:
            return -1
        self.counter(key)
        return self.vals[key]

    def put(self, key: int, value: int) -> None:
        if self.cap == 0:
            return

        if key not in self.vals and len(self.vals) == self.cap:
            res = self.cache[self.lfu_count].popLeft()
            self.vals.pop(res)
            self.count.pop(res)

        self.vals[key] = value
        self.counter(key)
        self.lfu_count = min(self.lfu_count, self.count[key])

# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)