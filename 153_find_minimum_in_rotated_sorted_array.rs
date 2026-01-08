impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {

        let (mut ans,mut mid,  mut low, mut high) = (nums[0], 0,0, nums.len()-1);


        while low < high { 
            mid = low + (high - low) / 2; 
            //println!("low: {:?}, high:  {:?}, mid: {:?}", low, high, mid ); 

            if nums[mid] < nums[low] && nums[mid] < nums[high] { 
                ans = nums[mid]; 
                if mid == 0 {break;}
                high = mid; 
            }else if nums[low] < nums[mid] && nums[low] < nums[high] { 
                ans = nums[low]; 
                if mid == 0{break;}
                high = mid;  
            }else { 
                ans = nums[high]; 
                low = mid; 
            }
            if high == low + 1 { 
                return  std::cmp::min(nums[high], nums[low]); 

            }

        }
        ans 
    }
}