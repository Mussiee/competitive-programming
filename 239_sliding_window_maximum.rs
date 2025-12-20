use std::collections::BTreeMap; 
use std::cmp::max;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut map = BTreeMap::new();
        let mut b = k as usize; 
        let mut ans: Vec<i32> = Vec::new();


        while b > 0 { 
            *map.entry(nums[b-1]).or_insert(0) += 1;
            b -= 1; 
        }
        b = k as usize; 
        let mut l = 0; 
        ans.push(*map.iter().next_back().unwrap().0); 
        
        for val in b..nums.len() { 
            *map.entry(nums[val]).or_insert(0) += 1; 
            if let Some(x) = map.get_mut(&nums[l]) { 
                if *x > 1 { 
                    *x -= 1; 
                }
                else { 
                    map.remove(&nums[l]);
                }
            }
            l += 1; 
            ans.push(*map.keys().next_back().unwrap()); 
        }




        ans
    }
}