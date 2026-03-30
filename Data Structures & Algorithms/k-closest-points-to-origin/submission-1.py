class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []
        res = []

        for p in points:
            heapq.heappush(heap, (-math.sqrt((p[0]**2) + (p[1]**2)), p))
            if len(heap) > k:
                heapq.heappop(heap)
        
        while heap:
            res.append(heapq.heappop(heap)[1])

        return res