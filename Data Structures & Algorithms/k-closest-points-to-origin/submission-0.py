class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = [(math.sqrt((n[0]**2) + (n[1]**2)), n) for n in points]
        heapq.heapify(heap)
        res = []

        for i in range(k):
            res.append(heapq.heappop(heap)[1])

        return res