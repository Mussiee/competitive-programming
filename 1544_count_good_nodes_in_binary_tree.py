# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:

            count = [0] 

            def gd(root, mn): 
                if root == None: 
                    return 
                
                if root.val >= mn:
                    count[0] += 1
                    mn = root.val 
                gd(root.left, mn)
                gd(root.right, mn)
            
            gd(root, root.val) 
            return count[0]