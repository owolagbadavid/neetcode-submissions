# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def dfs(root, m):
            nonlocal res
            if not root:
                return -1001

            l = dfs(root.left, m)
            r = dfs(root.right, m)

            m = root.val
            res = max(res, m, r)
            res = max(res, m, l)
            res = max(res, m, l+root.val)
            res = max(res, m, r+root.val)
            res = max(res, m, l+r+root.val)

            return max(l+root.val, r+root.val, root.val)
        
        res = root.val
        dfs(root, -1001)
        return res



