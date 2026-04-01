# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if k == 1: 
            return head

        
        n, counter = k-1, 0
        curr = head 
        hds, bb, b, f, ff= head,head, head, head, head

        while curr: 
            if n == 0: 
                while n != (k-1): 
                    if f == b: 
                        f = f.next
                        ff = f 
                    else:
                        ff = f.next
                        f.next = b 
                        b = f
                        f = ff 
                        n += 1
                if counter == 0: 
                    head = b
                else: 
                    hds.next = b 
                    hds = bb

                counter += 1
                hds.next = None
                bb, b, curr = f,f, f
            else:
                n -= 1
                curr = curr.next
        

        if f: 
            hds.next = f
        
        return head