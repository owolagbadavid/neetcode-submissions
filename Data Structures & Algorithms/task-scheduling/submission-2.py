class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        has = defaultdict(int)
        res = 0

        for t in tasks:
            has[t] += 1

        heap = []
        for k in has.keys():
            heapq.heappush(heap, [-has[k], k])
        q = deque([])

        while heap or q:
            res += 1
            if q and q[-1]:
                nex = q.popleft()
                if nex:
                    heapq.heappush(heap, nex)
            if heap:
                cur = heapq.heappop(heap)
                cur[0] += 1
                if cur[0] != 0:
                    y = n if not q else n - len(q)
                    while y:
                        q.append(None)
                        y -= 1
                    q.append(cur)
        return res
        
        
            
