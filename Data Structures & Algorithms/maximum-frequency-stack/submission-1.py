class FreqStack:

    def __init__(self):
        self.count = {}
        self.vals = defaultdict(list)
        self.max_count = 0 

    def push(self, val: int) -> None:
        count = 0
        if val in self.count:
            count = self.count[val] + 1
            self.count[val] = count
        else:
            count = 1
            self.count[val] = count
        self.vals[count].append(val)
        self.max_count = max(count, self.max_count)

    def pop(self) -> int:
        k = self.max_count
        res = self.vals[k].pop()
        self.count[res] -= 1
        if not self.vals[k]:
            self.max_count -= 1
        return res

# Your FreqStack object will be instantiated and called as such:
# obj = FreqStack()
# obj.push(val)
# param_2 = obj.pop()