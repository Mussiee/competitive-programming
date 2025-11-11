use std::collections::HashMap; 

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a1 = [0; 26]; 
        
        for c in s.chars() { 
            a1[(c as usize) - 97] += 1;  
        }

        for c in t.chars() { 
            a1[(c as usize) - 97] -= 1; 
        }

        a1.iter().filter(|x| **x == 0).count() == 26
    }

}