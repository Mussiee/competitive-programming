impl Solution {
    pub fn max_profit(nums: Vec<i32>) -> i32 {
        let mut prof = 0; 
        let mut mn = nums[0]; 

        for i in 1..nums.len() { 
            if nums[i] < mn {
                mn = nums[i]; 

            }
            else if nums[i] > mn { 
                prof += nums[i] - mn; 
                mn = nums[i]; 
            }
        } 
        prof

    }
}