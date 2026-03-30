class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        adj = defaultdict(lambda: deque([]))
        paths = 0
        res = []
        tickets.sort()

        for s, d in tickets:
            paths += 1
            adj[s].append(d)
        
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
