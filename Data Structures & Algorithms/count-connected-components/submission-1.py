class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        dsu = DSU(n)

        for n1, n2 in edges:
            dsu.union(n1, n2)
        res = 0
        ps = set()
        for i in range(n):
            p = dsu.find(i)
            if p not in ps:
                ps.add(p)
                res += 1
        return res
        

class DSU:
    def __init__(self, n):
        self.par = {}
        self.rank = {}

        for i in range(n):
            self.par[i] = i
            self.rank[i] = 0
        
    def union(self, n1, n2):
        p1, p2, = self.find(n1), self.find(n2)

        if p1 == p2:
            return False
        
        if self.rank[p1] > self.rank[p2]:
            self.par[p2] = p1
        elif self.rank[p2] > self.rank[p1]:
            self.par[p1] = p2
        else:
            self.par[p1] = p2
            self.rank[p1] += 1
        
        return True
    
    def find(self, n):
        p = self.par[n]
        while p != self.par[p]:
            self.par[p] = self.par[self.par[p]]
            p = self.par[p]
        return p