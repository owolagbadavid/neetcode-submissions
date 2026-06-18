class MyQueue:

    def __init__(self):
        self.stack = []
        self.front = None

    def push(self, x: int) -> None:
        self.stack.append(x)
        if not self.front:
            self.front = x

    def pop(self) -> int:
        self.front= None
        n = len(self.stack)
        string = ""
        for i in range(n-1):
            elem = self.stack.pop()
            string = str(elem) + string
            if i == n-2:
                self.front = elem
        res = self.stack.pop()
        for i in range(len(string)):
            self.stack.append(int(string[i]))
        return res

    def peek(self) -> int:
        return self.front

    def empty(self) -> bool:
        return len(self.stack) == 0
        


# Your MyQueue object will be instantiated and called as such:
# obj = MyQueue()
# obj.push(x)
# param_2 = obj.pop()
# param_3 = obj.peek()
# param_4 = obj.empty()