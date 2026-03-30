class MedianFinder:

    def __init__(self):
        self.largeHeap = []
        self.smallHeap = []

    def addNum(self, num: int) -> None:
        heapq.heappush(self.smallHeap, -num)
        if self.largeHeap and self.largeHeap[0] < -self.smallHeap[0]:
            heapq.heappush(self.largeHeap, -heapq.heappop(self.smallHeap))
        
        minLen = len(self.smallHeap)
        maxLen = len(self.largeHeap)
        
        if abs(minLen - maxLen) > 1:
            if minLen > maxLen:
                heapq.heappush(self.largeHeap, -heapq.heappop(self.smallHeap))
            else:
                heapq.heappush(self.smallHeap, -heapq.heappop(self.largeHeap))
        

    def findMedian(self) -> float:
        minLen = len(self.smallHeap)
        maxLen = len(self.largeHeap)

        if (minLen + maxLen) % 2:
            return self.largeHeap[0] if maxLen > minLen else -self.smallHeap[0]
        return (self.largeHeap[0] + -self.smallHeap[0]) / 2

        