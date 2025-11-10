impl Solution {
    pub fn asteroid_collision(nums: Vec<i32>) -> Vec<i32> {
        let mut stk:Vec<i32> = Vec::new(); 

        for val in nums.iter() { 
            if *val < 0 { 
                while stk.len() > 0 && stk[stk.len()-1] > 0 && (*val * -1) > stk[stk.len()-1] { 
                    stk.pop(); 
                }
                if stk.len() == 0 || stk[stk.len()-1] < 0 { 
                    stk.push(*val); 
                }
                else if stk[stk.len()-1] == *val * -1 { 
                    stk.pop(); 
                }
            }
            else { 
                stk.push(*val); 
            }
        }

        stk 
    }
}