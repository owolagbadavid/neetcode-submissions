class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = [-n for n in stones]
        heapq.heapify(heap)
        while len(heap) > 1:
            first, second = -heapq.heappop(heap), -heapq.heappop(heap)
            if first > second:
                heapq.heappush(heap, -(first-second))
            elif second > first:
                heapq.heappush(heap, -(second-first))
        return -heap[-1] if heap else 0