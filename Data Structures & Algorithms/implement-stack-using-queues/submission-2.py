class MyStack:

    def __init__(self):
        self.q = deque([])
        self.size = 0

    def push(self, x: int) -> None:
        self.q.append(x)
        self.size += 1

    def pop(self) -> int:
        q2 = deque([])
        self.size -= 1
        for i in range(self.size):
            q2.append(self.q.popleft())
        res = self.q.popleft()
        self.q = q2
        return res

    def top(self) -> int:
        return self.q[-1]

    def empty(self) -> bool:
        return len(self.q) == 0
        


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()