use std::collections::HashMap; 

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut test = HashMap::new();  
        for val in nums.iter() {
            match test.insert(val,0) { 
                None => (),  
                Some(x) => return true, 
            }
        }

       false 
    }
}