use std::collections::BTreeMap; 
use std::cmp::max;
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut ans = Vec::new();  

        let mut dq = VecDeque::new(); 
        let (mut l , mut r) = (0, 0); 

        while r < nums.len() { 
            
            //remvoe elements that are less than the current element from the dq
            while dq.len() > 0 && nums[dq[dq.len()-1]] < nums[r] { 
                dq.pop_back(); 
            }

            //insert the element's index
            dq.push_back(r);  

            // check if the left element of the dq is out of bounds 
            if (r - l)  >= (k-1) as usize { 
                ans.push(nums[dq[0]]); 
                 if dq[0] <= l {
                    dq.pop_front(); 
                 }

                l += 1; 
            }
            
            r += 1; 
        }


        ans
    }
}