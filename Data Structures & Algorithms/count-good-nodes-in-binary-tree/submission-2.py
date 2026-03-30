# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        res = 0
        def dfs(root, num):
            nonlocal res
            if not root:
                return
            if root.val >= num:
                res += 1
            
            dfs(root.left, max(num, root.val))
            dfs(root.right, max(num, root.val))
        
        dfs(root, -101)
        return res
        