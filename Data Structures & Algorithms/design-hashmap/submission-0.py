class MyHashMap:

    def __init__(self):
        self.size = 0
        self.cap = 10
        self.data = [[]]*10

    def put(self, key: int, value: int) -> None:
        i = self.hash(key)
        self.tryRemove(i, key)
        self.data[i].append((key, value))
        self.size += 1
        if self.size == self.cap:
            self.resize()

    def get(self, key: int) -> int:
        i = self.hash(key)
        try:
            return [x for x in self.data[i] if x[0] == key][0][1]
        except:
            return -1

    def remove(self, key: int) -> None:
        i = self.hash(key)
        self.tryRemove(i, key)

    def tryRemove(self, index, key):
        self.data[index] = [x for x in self.data[index] if x[0] != key]
    
    def hash(self, key) -> int:
        return key % self.cap

    def resize(self) -> bool:
        self.cap *= 2
        data = self.data
        self.data = [[]]*self.cap
        self.size = 0
        for d in data:
            for k, v in d:
                self.push(k, v)


# Your MyHashMap object will be instantiated and called as such:
# obj = MyHashMap()
# obj.put(key,value)
# param_2 = obj.get(key)
# obj.remove(key)

