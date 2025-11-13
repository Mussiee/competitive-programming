use std::collections::{HashMap, HashSet}; 

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
       //lets first try the brute force which is we put every string in a hashmap and then store eventhing 
       //together in a vector and then iterate the vector twice for each element to find all the elements that are similar 

        let mut v: Vec<HashMap<char, i32>> = strs.iter().
            map(|value| 
                value.chars().fold(HashMap::new(), |mut mp, c| {
                    *mp.entry(c).or_insert(0) += 1; 
                    mp
                })
            ).collect(); 

        let mut ans = Vec::new(); 
        let mut checked: HashSet<usize> = HashSet::new(); 

        for i in 0..v.len() { 
            let mut temp = Vec::new(); 
             for j in (i+1)..v.len() { 
                if v[i] == v[j] && !checked.contains(&j) { 
                    temp.push(strs[j].clone()); 
                    checked.insert(j);
                }
             }
            if !checked.contains(&i)  { 
                temp.push(strs[i].clone());  
            }
            checked.insert(i); 
            if temp.len() > 0 {
                ans.push(temp); 
            }
        }
        ans
    }
}