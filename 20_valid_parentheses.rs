impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk = Vec::new(); 
        
        for v in s.chars() { 
            if v == '(' || v == '{' || v == '[' { 
                stk.push(v); 
            }
            else  { 
                if v == '}' { 
                    match stk.pop() { 
                        Some(x) => if x != '{' { 
                            return false; 
                        }
                        _ => {return false;}
                    } 
                }
                else if v == ']' { 
                    match stk.pop() { 
                        Some(x) => if x != '[' { 
                            return false; 
                        }
                        _ => {return false;}
                    } 

                }
                else { 
                    match stk.pop() { 
                        Some(x) => if x != '(' { 
                            return false; 
                        }
                        _ => {return false;}
                    } 

                }
            }
        }

        if stk.len() == 0{ 
            return true;
        }
  
        false
    }
}