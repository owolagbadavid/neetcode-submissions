class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        adj = defaultdict(list)
        paths = 0
        res = []
        visit = set()

        for s, d in tickets:
            paths += 1
            adj[s].append(d)
        for k in adj:
            adj[k] = deque(sorted(adj[k]))
        
        def dfs(source, arr):
            nonlocal paths
            arr.append(source)
            if paths == 0:
                res.extend(arr)
                return True
    
            for i in range(len(adj[source])):
                dest = adj[source].popleft()
                paths -= 1
                if dfs(dest, arr):
                    return True
                adj[source].append(dest)
                paths += 1
            arr.pop()
            return False

        dfs('JFK', [])
        return res
