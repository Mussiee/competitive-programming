use std::collections::HashMap; 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // CAN BE SOLVED WITH ONLY ONE HASHMAP O(N) time this is a different approach 
        let mut mp: HashMap<i32, Vec<i32>>  = HashMap::new();
        let mut v = Vec::from(nums); 
        let mut ans = Vec::new(); 
        let mut l = 0; 
        let mut r = (v.len() - 1); 

        for i in 0..v.len() { 
            mp.entry(v[i]).or_insert(Vec::new()).push(i as i32); 
        } 
        v.sort(); 

        while l <= r { 
            let temp = v[l] + v[r]; 
            if temp == target { 

                if let Some(val) = mp.get_mut(&v[l]) { 
                    if let Some(second_val) = val.pop() {
                        ans.push(second_val); 
                    }
                } 
                if let Some(val) = mp.get_mut(&v[r]) { 
                    if let Some(second_val) = val.pop() { 
                        ans.push(second_val); 
                    }
                }
                break; 
            }
            else if temp < target { 
                l += 1; 
            } 
            else { 
                r -= 1; 
            }
        }
        ans
    }
}