class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        adj = { i: [] for i in range(n) }
        for fro, to, p in flights:
            adj[to].append((p, fro))
        
        heap = [(0, dst, k)]
        visit = set()

        while heap:
            price, node, stop = heapq.heappop(heap)
            if node in visit:
                continue
            if node == src:
                return price
            if stop < 0:
                continue
            stop -= 1
            for price1, node1 in adj[node]:
                if node1 not in visit:
                    heapq.heappush(heap, (price1+price, node1, stop))
        return -1