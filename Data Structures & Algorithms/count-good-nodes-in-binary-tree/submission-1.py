# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode, maxi=-101) -> int:
        if not root:
            return 0
        
        good = 0
        if root.val >= maxi:
            maxi = root.val
            good = 1
        
        return good + (self.goodNodes(root.right, maxi) + self.goodNodes(root.left, maxi))