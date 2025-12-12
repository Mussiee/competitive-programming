impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut mx = 0; 
        let mut counter = 0; 
        let mut v: Vec<i32> = (0..26).map(|_| 0).collect();  
        let mut ss: Vec<char> = s.chars().collect();
        let mut l = 0; 

        fn is_valid(v: &Vec<i32>, k: i32) -> bool { 
            let mut sm = 0;
            let mx = v.iter().max().unwrap(); 
            let mut c = 1; 

            for val in v.iter() { 
                if *val == *mx { 
                    if c == 0 { 
                        sm += *val;
                    }
                    else { 
                        c -= 1;
                        continue; 
                    }
                }
                else { 
                    sm += *val;
                }
            }
            if sm <= k { 
                return true; 
            }
            false
        }

        for c in ss.iter() { 
            mx = std::cmp::max(mx, counter); 
            v[((*c as u8)-65) as usize] += 1; 
            if is_valid(&v, k) { 
                counter += 1; 
            }
            else { 
                while l < ss.len() && !(is_valid(&v, k)) { 
                    v[((ss[l] as u8)-65) as usize] -= 1; 
                    l += 1; 
                    counter -= 1; 
                }    
                counter += 1; 
            }

        } 
        mx = std::cmp::max(mx, counter);  
        mx  
    }
}