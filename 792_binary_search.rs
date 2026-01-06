impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let (mut l, mut h) = (0, nums.len() - 1);         

        while l < h { 
            let mid = l + (h - l) / 2; 
            
            if nums[mid] > target { 
                h = mid;
            } else if nums[mid] < target {

                l = mid + 1; 

            }else { 
                return mid as i32; 
            }
        }
        -1
    }
}