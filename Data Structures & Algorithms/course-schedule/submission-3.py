class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        grp = defaultdict(list)
        for x, y in prerequisites:
            grp[x].append(y)
        
        visited = set()
        def dfs(n):
            if n in visited:
                return True
            if n not in grp:
                return False
            
            visited.add(n)

            for y in grp[n]:
                if dfs(y):
                    return True

            visited.remove(n)
            return False
        
        for k in grp:
            if dfs(k):
                return False

        return True
 