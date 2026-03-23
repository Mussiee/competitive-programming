# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:

        fast, slow, prev = head, head, None 

        while n > 0 and fast: 
            fast = fast.next
            n -= 1

        while fast: 
            prev = slow
            slow = slow.next
            fast = fast.next 
        

        #print(slow.val)
        if prev: 
            prev.next = slow.next
            slow.next = None
        else: 
            return head.next
        return head