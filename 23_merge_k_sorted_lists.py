import heapq
# Definition for singly-linked list.
class Wrapper:
    def __init__(self, node):
        self.node = node
    def __lt__(self, other):
        return self.node.val <= other.node.val

class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:

        #nm = []
        nums = []
        for i in range(len(lists)): 
            if lists[i]: 
                temp = lists[i] 
                while temp:
            #        nm.append(temp.val)
                    nums.append(Wrapper(temp))
                    temp = temp.next
                temp2 = lists[i]
                temp = lists[i]
                while temp: 
                    temp = temp.next
                    temp2.next = None
                    temp2 = temp
                    
        
        heapq.heapify(nums)
#        print(nm)
        dummy = ListNode(0)
        curr = dummy

        for i in range(len(nums)): 
            nd = heapq.heappop(nums).node
            curr.next = nd 
            curr = curr.next
        
        return dummy.next