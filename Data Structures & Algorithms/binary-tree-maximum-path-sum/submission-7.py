# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        def dfs(root):
            nonlocal res
            if not root:
                return -1001

            l = dfs(root.left)
            r = dfs(root.right)

            res = max(res, root.val, l+root.val)
            res = max(res, root.val, r+root.val)
            res = max(res, root.val, l+r+root.val)

            return max(l+root.val, r+root.val, root.val)
        
        res = root.val
        dfs(root)
        return res
