# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:

        def dfs(root, r, l): 

            if root == None: 
                return True
            if root.val >= r or root.val <= l: 
                return False
            
            return dfs(root.left, root.val, l) and dfs(root.right,r,root.val)
        
        return dfs(root, float("inf"), float("-inf"))