impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        
        
        let mut st = s.to_lowercase(); 

        let mut v = Vec::new(); 

        for c in st.chars() { 
            if c as u8 >= 97 && c as u8 <= 122 { 
                v.push(c); 
            }
            if c as u8 >= 48 && c as u8 <= 57 { 
                v.push(c); 
            }
        }
        let mut l = 0; 
        let mut r = v.len()-1; 
        if v.len() == 0 { 
            return true; 
        }
        while l < r { 
            if v[l] != v[r] { 
                return false; 
            }
            l += 1; 
            r -= 1; 
        }

        true 
    }
}