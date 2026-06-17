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
            nonlocal res
            if not root:
                return -1001
            l = dfs(root.left)
            r = dfs(root.right)
            res = max(res, root.val, root.val + l, root.val + r, root.val + r + l)

            return max(root.val, root.val + l, root.val + r)
        dfs(root)
        return res
