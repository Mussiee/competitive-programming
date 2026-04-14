# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':


        s1 = set()
        flag = [False]
        ret = [-1]
        

        def i(root): 
            if root == None:
                return 
            s1.add(root)
            if p.val < root.val: 
                i(root.left) 
            elif p.val > root.val: 
                i(root.right)
            else:
                return 
        
        def i2(root): 
            if root == None: 
                return 
            if root in s1: 
                ret[0] = root
            if q.val < root.val: 
                i2(root.left) 
            elif q.val > root.val: 
                i2(root.right)
            else: 
                return 

        i(root) 
        i2(root)
        return ret[0]