class MyHashSet:

    def __init__(self):
        self.size = 0
        self.cap = 10
        self.data = [[]]*10
        
    def add(self, key: int) -> None:
        i = self.hash(key)
        self.tryRemove(i, key)
        self.data[i].append(key)
        self.size += 1
        if self.size == self.cap:
            self.resize()

    def remove(self, key: int) -> None:
        i = self.hash(key)
        self.tryRemove(i, key)

    def contains(self, key: int) -> bool:
        i = self.hash(key)
        return key in self.data[i]

    def resize(self) -> bool:
        self.cap *= 2
        data = self.data
        self.data = [[]]*self.cap
        self.size = 0
        for d in data:
            for k in d:
                self.add(k)

    def hash(self, key) -> int:
        return key % self.cap
    
    def tryRemove(self, index, key):
        try:
            self.data[index].remove(key)
            self.size -= 1
        except ValueError:
            pass


# Your MyHashSet object will be instantiated and called as such:
# obj = MyHashSet()
# obj.add(key)
# obj.remove(key)
# param_3 = obj.contains(key)