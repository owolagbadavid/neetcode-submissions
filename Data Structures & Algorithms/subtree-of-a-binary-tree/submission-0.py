# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution: 
    def __init__(self):
        self.same = False  
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def isSame(root, subRoot):
            if root is None and subRoot is None:
                return True
            
            if root is None or subRoot is None:
                return False
            
            lRes = isSame(root.left, subRoot.left)
            rRes = isSame(root.right, subRoot.right)
        
            return root.val == subRoot.val and lRes and rRes

        if root is None and subRoot is None:
            return True
        
        if root is None:
            return False
        
        self.same = self.same or isSame(root, subRoot)

        if not self.same:
            self.isSubtree(root.right, subRoot)
            self.isSubtree(root.left, subRoot)
        
        return self.same
        
            
