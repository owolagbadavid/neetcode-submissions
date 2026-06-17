class MyStack:

    def __init__(self):
        self.q = deque()
        self.size = 0
        self._top = None

    def push(self, x: int) -> None:
        self.q.append(x)
        self.size += 1
        self._top = x

    def pop(self) -> int:
        self.size -= 1
        for _ in range(self.size-1):
            self.q.append(self.q.popleft())
        if self.size == 0:
            self._top = None
        else:
            self._top = self.q.popleft()
            self.q.append(self._top)
        res = self.q.popleft()
        return res

    def top(self) -> int:
        return self._top

    def empty(self) -> bool:
        return self.size == 0


# Your MyStack object will be instantiated and called as such:
# obj = MyStack()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.top()
# param_4 = obj.empty()