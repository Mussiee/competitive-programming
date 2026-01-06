impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {

        fn add(arr: &[i32], val: i32, h: f64)-> bool{ 

            let mut curr = 0 as f64;
            for v in arr.iter() { 
                curr += (*v as f64 /  val as f64).ceil(); 
                if curr > h { 
                    return false;
                }
            } 
            curr <= h 
        } 

        let mut high = *piles.iter().max().unwrap();
        let mut low = 0;
        let mut last= high;

        while low <= high { 
            let mid = low + (high - low) / 2; 
            if  add(&piles, mid , h as f64) { 
                last = mid;
                high = mid - 1; 
            }else { 
                low = mid + 1;
            }
            //println!("{:?}", mid); 
        }

       last 

    }
}