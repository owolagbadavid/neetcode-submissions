class StockSpanner:

    def __init__(self):
        self.stack = []

    def next(self, price: int) -> int:
        n = len(self.stack)
        self.stack.append(price)
        if n == 0 or self.stack[-2] > price:
            print(self.stack)
            return 1
        i = n
        while i >= 0:
            if self.stack[i] > price:
                break
            i -= 1
        return n-i
        


# Your StockSpanner object will be instantiated and called as such:
# obj = StockSpanner()
# param_1 = obj.next(price)