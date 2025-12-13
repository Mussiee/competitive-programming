use std::collections::HashMap; 
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() { 
            return false;
        }
        let mut h1 = HashMap::new();   
        let mut h2 = HashMap::new(); 
        let mut v: Vec<char> = s2.chars().collect();
        let (mut l, mut r) = (0, s1.len()); 
        
        for c in s1.chars() { 
            *h1.entry(c).or_insert(0) += 1; 
        }

        let mut temp = (s1.len() + 1)as i32 ;  
        for c in 0..v.len() { 
            *h2.entry(v[c]).or_insert(0) += 1; 
            temp -= 1; 
            if temp < 1 { 
                if let Some(x) = h2.get_mut(&v[l]) { 
                    if *x > 1 { 
                        *x -= 1; 
                    } 
                    else { 
                     h2.remove(&v[l]);
                    }
                }
                l += 1; 
            }
                if h1 == h2 { 
                    return true;
                }

        }

        


        false
    }
}