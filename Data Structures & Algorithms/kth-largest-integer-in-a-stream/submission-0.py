class KthLargest:

    def __init__(self, k: int, nums: List[int]):
        self.nums = [-n for n in nums]
        self.k = k
        heapq.heapify(self.nums)

    def add(self, val: int) -> int:
        heapq.heappush(self.nums, -val)
        store = []
        for i in range(self.k):
            store.append(heapq.heappop(self.nums))
        res = -store[-1]
        for n in store:
            heapq.heappush(self.nums, n)
        return res
