class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        mapping = {}
        for i in range(numCourses):
            mapping[i] = []
        for x, y in prerequisites:
            mapping[x].append(y)
        
        res = []
        def dfs(n, visited):
            if n in visited:
                return True

            if mapping[n] == None:
                return False
            
            visited.add(n)
            
            for x in mapping[n]:
                if dfs(x, visited):
                    return True
            res.append(n)
            mapping[n] = None

            visited.remove(n)
            return False

        for k in mapping:
            if dfs(k, set()):
                return []

        return res