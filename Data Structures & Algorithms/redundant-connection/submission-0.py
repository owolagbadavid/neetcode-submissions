class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        dsu = DSU(len(edges))
        for x, y in edges:
            if not dsu.union(x, y):
                res = [x, y]
        return res

class DSU:
    def __init__(self, n):
        self.par = {}
        self.rank = {}

        for i in range(n):
            self.par[i+1] = i+1
            self.rank[i+1] = 0
    
    def union(self, n1, n2):
        p1, p2 = self.find(n1), self.find(n2)

        if p1 == p2:
            return False
        
        if self.rank[p1] > self.rank[p2]:
            self.par[p2] = p1
        if self.rank[p2] > self.rank[p1]:
            self.par[p1] = p2
        else:
            self.par[p1] = p2
            self.rank[p2] += 1
        
        return True
    
    def find(self, n):
        p = n
        while self.par[p] != p:
            self.par[p] = self.par[self.par[p]]
            p = self.par[p]

        return p