class MinStack:

    def __init__(self):
        self.mainStack = []
        self.minStack = []
        self.mini = float('inf')

    def push(self, val: int) -> None:
        self.mainStack.append(val)
        self.mini = min(self.mini, val)
        self.minStack.append(self.mini)

    def pop(self) -> None:
        self.mainStack.pop()
        self.minStack.pop()
        self.mini = self.minStack[-1] if len(self.minStack) else float('inf')

    def top(self) -> int:
        return self.mainStack[-1]

    def getMin(self) -> int:
        return self.mini
