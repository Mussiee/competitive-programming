# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        
        mx = [0]

        def dia(root): 

            if root == None: 
                return 0 
            
            left =  dia(root.left)
            right = dia(root.right) 

            mx[0] = max(mx[0], left + right)
            return max(left, right) + 1
        

        dia(root)
        return mx[0]