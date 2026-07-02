class MyCircularQueue:

    def __init__(self, k: int):
        self.size = 0
        self.cap = k
        self.head = None

    def enQueue(self, value: int) -> bool:
        if self.isFull():
            return False
        cur = ListNode(value)
        if self.head:
            cur.next = self.head.next 
            self.head.next.prev = cur
        else:
            self.head = cur
        self.head.next = cur
        self.size += 1
        return True
        

    def deQueue(self) -> bool:
        if self.isEmpty():
            return False
        if self.size == 1:
            self.head = None
        else:
            tmp = self.head.next
            self.head = self.head.prev
            self.head.next = tmp
        self.size -= 1
        return True
        

    def Front(self) -> int:
        if not self.isEmpty():
            return self.head.val
        else:
            return -1
        

    def Rear(self) -> int:
        if not self.isEmpty():
            return self.head.next.val
        else:
            return -1
    

    def isEmpty(self) -> bool:
        return self.size == 0
        

    def isFull(self) -> bool:
        return self.size == self.cap
        

class ListNode:
    def __init__(self, val=0, next=None, prev=None):
        self.val = val
        self.next = next
        self.prev = prev

# Your MyCircularQueue object will be instantiated and called as such:
# obj = MyCircularQueue(k)
# param_1 = obj.enQueue(value)
# param_2 = obj.deQueue()
# param_3 = obj.Front()
# param_4 = obj.Rear()
# param_5 = obj.isEmpty()
# param_6 = obj.isFull()