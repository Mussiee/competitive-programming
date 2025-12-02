use std::cmp::max; 
use std::collections::HashMap; 

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut curr = 0;  
        let mut hs = HashMap::new(); 

        for val in nums { 
            hs.insert(val, 1); 
        }

        let keys: Vec<i32> = hs.keys().cloned().collect();
        for key in keys { 

            if hs.contains_key(&(key+1)) { 
                let mut temp = key+1; 
                while hs.contains_key(&temp) { 
                    if let Some(x) = hs.get(&temp)  { 
                        curr += x;  
                    }
                    hs.remove(&temp); 
                    temp += 1;
                }
            }
            if let Some(x) = hs.get_mut(&key) { 
                *x += curr; 
                curr = 0; 
            }
        }
        if let Some(ans) = hs.values().max()  { 
            return *ans; 
        }
        0
    }
}