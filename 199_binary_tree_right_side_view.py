# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        q = deque() 
        if root == None: 
            return []
        q.append(root) 
        ans = []

        while q: 
            temp = []
            l = len(q)
            for _ in range(l): 
                curr = q.popleft()
                temp.append(curr.val)
                if curr.left: 
                    q.append(curr.left)
                if curr.right: 
                    q.append(curr.right)  
            
            ans.append(temp[-1])
        
        return ans