class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        
        slow, fast = nums[0], nums[0]

        while nums[fast]: 
            slow = nums[slow] 
            fast = nums[nums[fast]]
            if fast == slow: 
                break 
        
        fast = nums[0]
        while nums[slow]:
            if fast == slow: 
                return slow
            fast = nums[fast]
            slow = nums[slow] 
        

        return -1

1