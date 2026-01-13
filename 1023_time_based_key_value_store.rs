use std::collections::HashMap; 
struct TimeMap {
    hs: HashMap<String, Vec<(i32, String)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap { 
            hs: HashMap::new()
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let x = self.hs.get_mut(&key); 
        match x {
            Some(v) => v.push((timestamp, value)),
            None => {
                self.hs.insert(key,vec![(timestamp, value)]); 
            }
        }; 
        //println!("{:?}", self.hs); 
    }
    
    fn get(&mut self, key: String, timestamp: i32) -> String {
        let tmp = self.hs.get_mut(&key); 
        let mut ans = String::new(); 
        if let Some(nums) = tmp { 

            let (mut low, mut high) = (0, nums.len()-1); 
            while low <= high { 
                let mid = low + (high - low) / 2; 
                if nums[mid].0 > timestamp { 
                    if mid == 0 {break;}
                    high = mid - 1; 
                }else if nums[mid].0 <= timestamp { 
                    low = mid + 1; 
                    ans = nums[mid].1.clone(); 
                }
            }
        }; 
        ans
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */