# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        res = -1001
        def dfs(root):
            if not root:
                return -1001
            
            nonlocal res
            lVal = dfs(root.left)
            rVal = dfs(root.right)
            total = root.val
            res = max(res, total, total + lVal, total + rVal, total + lVal + rVal)
        
            return max(total, total, total + lVal, total + rVal,)

        dfs(root)
        return res