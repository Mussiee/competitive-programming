impl Solution {
    pub fn daily_temperatures(nums : Vec<i32>) -> Vec<i32> {
        let mut stk = Vec::new(); 
        let mut ans = vec![0; nums.len()]; 

        for i in  0..nums.len() { 
           while stk.len() > 0 && nums[i] > nums[stk[stk.len()-1] as usize] { 
                ans[stk[stk.len()-1] as usize] = (i as i32)-stk[stk.len()-1]; 
                stk.pop(); 
           } 
           stk.push(i as i32); 
        }

        ans
    }
}