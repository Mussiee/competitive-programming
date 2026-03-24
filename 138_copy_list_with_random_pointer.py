class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        #this is even much better than the first two 
        if not head:
            return None

        nw = {}

        # create all nodes
        curr = head
        while curr:
            nw[curr] = Node(curr.val)
            curr = curr.next

        # assign next and random
        curr = head
        while curr:
            copy = nw[curr]
            copy.next = nw.get(curr.next)
            copy.random = nw.get(curr.random)
            curr = curr.next

        return nw[head]