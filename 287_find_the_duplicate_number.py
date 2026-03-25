class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        

        for i in range(len(nums)): 

            
            if nums[i] < 0: 
                if nums[(nums[i] * -1)] < 0: 
                    return nums[i] * -1 
                nums[(nums[i] * -1)] *= -1
            else: 
                if nums[nums[i]] < 0: 
                    return nums[i]
                nums[nums[i]] *= -1
        
        return -1