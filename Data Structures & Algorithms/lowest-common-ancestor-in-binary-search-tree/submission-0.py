# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:

        def dfs(root):
            if not root:
                return

            nonlocal res
            res = root
            
            if p.val < root.val and q.val < root.val:
                return dfs(root.left)
            if p.val > root.val and q.val > root.val:
                return dfs(root.right)
            
            return root

        res = root
        dfs(root)
        return res
        
