use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new(); 
        for val in nums.iter() { 
            *mp.entry(*val).or_insert(0) += 1; 
        }

        let mut ans: Vec<Vec<i32>> = mp.iter().map(|(&x, &y)| vec![x,y]).collect();
        ans.sort_by(|a, b| a[1].cmp(&b[1]));

        let x: Vec<Vec<i32>> = ans[(ans.len() - k as usize)..].to_vec();
        x.iter().map(|val| val[0]).collect()
    }
}