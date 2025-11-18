impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        
        let mut pref = Vec::new(); 
        pref.push(1); 
        let mut curr = 1; 
        let mut left = 1; 

        for val in nums.iter().rev() { 
            curr = curr * val; 
            pref.push(curr); 

        }
        let mut temp = pref.len()-2; 
        curr = 1; 

        let mut ans = Vec::new();
        for i in 0..nums.len() { 
            ans.push(curr * pref[temp]); 
            curr *= nums[i]; 
            temp -= 1; 
        } 


        
        ans
    }
}