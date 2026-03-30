class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        adj = defaultdict(list)
        total = 0
        for i in range(len(points)):
            xi, yi = points[i]
            for j in range(i+1, len(points)):
                xj, yj = points[j]
                adj[(xi, yi)].append([abs(xi-xj)+abs(yi-yj), (xj, yj)])
                adj[(xj, yj)].append([abs(xi-xj)+abs(yi-yj), (xi, yi)])

        heap = [(0, (points[0][0], points[0][1]))]
        visit = set()

        while heap:
            cur = heapq.heappop(heap)
            if cur[1] in visit:
                continue
            visit.add(cur[1])
            total += cur[0]
            if len(visit) == len(points):
                return total
            for cost, (x, y) in adj[cur[1]]:
                heapq.heappush(heap, (cost, (x,y)))        

        return total