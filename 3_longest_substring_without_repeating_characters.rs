use std::collections::HashSet; 
use std::cmp::max; 

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h = HashSet::new();     
        let mut mx = 0;
        let mut l = 0;
        let mut v: Vec<char> = Vec::new();

        for c in s.chars() { 
            if h.contains(&c) { 
                while v[l] != c { 
                    h.remove(&v[l]); 
                    l += 1; 
                }
                h.remove(&v[l]); 
                l += 1; 
            }
            h.insert(c);
            v.push(c);
            mx = max(mx, h.len()); 
        } 
       

        mx as i32

    }
}