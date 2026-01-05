impl Solution {
    pub fn largest_rectangle_area(nums: Vec<i32>) -> i32 {
       //[4,3,5,5,9,2,8,4,7,2,3,8,3,5,4,7,9]
        //[2,1,2]
        //[3,6,5,7,4,8,1,0]

        let mut stk = Vec::new(); 
        let mut sm_r = vec![0; nums.len()]; 
        let mut sm_l = vec![0; nums.len()];

        // next smaller element to the right
        for i in 0..nums.len() { 
            while !stk.is_empty() && nums[i] < nums[*stk.last().unwrap()] { 
                let idx = stk.pop().unwrap(); 
                sm_r[idx] = i as i32; // store the smaller element for that index
            }
            stk.push(i);  
        }
        while !stk.is_empty() { 
            sm_r[stk.pop().unwrap()] = nums.len() as i32; 
        }

        stk.clear(); 

        // Next smaller element to the left 
        for i in (0..nums.len()).rev() { 
            while !stk.is_empty() && nums[i] < nums[*stk.last().unwrap()] { 
                let idx = stk.pop().unwrap(); 
                sm_l[idx] = i as i32; 
            }
            stk.push(i);
        }
        while !stk.is_empty() { 
            sm_l[stk.pop().unwrap()] = -1;
        }


        let mut mx = 0; 
        for i in 0..nums.len() { 
            let (mut l, mut r) = (sm_l[i], sm_r[i]); 
            //println!("l: {:?}, r: {:?}", l , r); 
            mx = std::cmp::max(mx, (r - l - 1) * nums[i]); 
        }

       //println!("{:?},          {:?}", sm_r, sm_l); 
        mx as i32 
    }
}