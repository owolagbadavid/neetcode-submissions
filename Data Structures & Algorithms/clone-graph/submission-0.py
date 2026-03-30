"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:

        mapp = {}
        def dfs(node):
            if not node:
                return
            
            if node.val in mapp:
                return mapp[node.val]
            else:
                copy = Node(node.val)
                mapp[node.val] = copy
            
            for n in node.neighbors:
                neighbor = dfs(n)
                if neighbor:
                    copy.neighbors.append(neighbor)
            return copy
        return dfs(node)
            

        

        