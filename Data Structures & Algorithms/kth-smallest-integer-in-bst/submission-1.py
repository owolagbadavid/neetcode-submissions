# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:

        left = k
        def dfs(root):
            if not root:
                return 0
            
            l = dfs(root.left)
            if l:
                return l
            nonlocal left
            left -= 1
            if left == 0:
                return root.val
            r = dfs(root.right)

            return l if l else r

        return dfs(root)