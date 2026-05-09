class Solution:
    def shortestPath(self, n: int, edges: List[List[int]], src: int) -> Dict[int, int]:
        adj = defaultdict(list)
        res = {}
        for s, d, c in edges:
            adj[s].append((c, d))
        for i in range(n):
            res[i] = -1

        heap = [(0, src)]
        while heap:
            cost, dest = heapq.heappop(heap)
            if res[dest] != - 1:
                continue
            res[dest] = cost
            for c, d in adj[dest]:
                if res[d] == -1:
                    heapq.heappush(heap, (cost+c, d))
        
        return res
