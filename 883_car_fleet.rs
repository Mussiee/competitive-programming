use std::collections::HashMap;

impl Solution {
    pub fn car_fleet(target: i32, mut position: Vec<i32>, speed: Vec<i32>) -> i32 {

        let mut v: Vec<(i32, f32)>  = Vec::new(); 
        let (mut curr, mut mx,) = (0.0, 1); 
        for i in 0..position.len() { 
            v.push((position[i], 0.0)); 
            v[i].1 = (target as f32 - position[i]  as f32) / (speed[i] as f32); 
        }
        v.sort_by(|a, b| b.0.cmp(&a.0)); 
        //println!("{:?}", v); 

        curr =  v[0].1; 
        for i in 1..position.len() { 
            if v[i].1 > curr { 
                mx += 1; 
                curr = v[i].1; 
            }

        }
        mx  
    }
}