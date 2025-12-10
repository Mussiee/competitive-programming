use std::cmp::max;
impl Solution {
    pub fn max_profit(nums: Vec<i32>) -> i32 {
       let mut mx = 0; 
       let mut mn = nums[0]; 

       for i in 1..nums.len() { 
            if nums[i] < mn {
                mn = nums[i]
             }
            else if nums[i] > mn { 
               mx = max(mx, nums[i] - mn);  
            }
            
       }
        
        mx 
    }
}