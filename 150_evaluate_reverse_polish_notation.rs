impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk: Vec<i32> = Vec::new(); 


        fn parse_string(s: &str)-> i32 { 

            match s.parse::<i32>() { 
                Ok(x) => return x, 
                Err(e) => return 201
            }
        }

        for val in tokens.iter() { 

            let temp = parse_string(&val); 

            if temp == 201 { 

                let (mut a, mut b) = (stk.pop().unwrap().to_owned(), stk.pop().unwrap().to_owned()); 

                if val == "+" { 
                    stk.push(b + a); 
                }else if val == "-" { 
                    stk.push(b- a); 
                }else if val == "*" { 
                    stk.push(b * a); 
                } else { 
                    stk.push(b/a); 
                }

            }

            else { 
                stk.push(temp); 
            }
        }

        stk[stk.len()-1]

    }
}