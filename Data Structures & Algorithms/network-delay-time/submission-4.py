class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        adj = defaultdict(list)
        for u, v, t in times:
            adj[u].append([t, v])

        dist = [float('inf')]*n
        heap = [[0, k]]
        visit = set()

        while heap:
            time, u = heapq.heappop(heap)
            dist[u-1] = time
            visit.add(u)
            if len(visit) == n:
                return max(dist)
            for t, v in adj[u]:
                if v not in visit:
                    heapq.heappush(heap, [t+time,v])
        return -1

            