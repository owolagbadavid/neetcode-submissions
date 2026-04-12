# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def dfs(root, lMin, rMax):
            if not root:
                return True
            
            if root.val >= lMin or root.val <= rMax:
                return False
            
            return dfs(root.left, min(root.val, lMin), rMax) and dfs(root.right, lMin, max(root.val, rMax))
    
        return dfs(root, 1001, -1001)