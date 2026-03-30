# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def __init__(self):
        self.maxi = 0
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        self.getHeights(root)
        return self.maxi
    
    def getHeights(self, root: Optional[TreeNode]):
        if root is None:
            return 0

        lHeight = self.getHeights(root.left)
        rHeight = self.getHeights(root.right)
        
        self.maxi = max(self.maxi, lHeight + rHeight)

        return 1 + max(lHeight, rHeight)
