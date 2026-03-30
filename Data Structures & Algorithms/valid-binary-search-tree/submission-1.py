# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode], small=-1001, big=1001) -> bool:
        if root is None:
            return True

        if root.val > small and root.val < big:
            pass # we good
        else:
            return False

        return self.isValidBST(root.left, small, min(big, root.val)) and self.isValidBST(root.right, max(small, root.val), big)
