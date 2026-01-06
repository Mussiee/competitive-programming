impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        fn binary_search(arr: &[i32], t: i32)->bool { 

            let (mut l , mut h) = (0, arr.len()-1); 

            while l <= h { 

                let m = l + (h - l) / 2; 

                if arr[m] > t { 
                   if m == 0{ break; } 
                   h = m -1; 
                }else if arr[m] < t{ 
                    l = m + 1; 
                }else { 
                    return true; 
                }

            }
            false 
        }

        for val in matrix.iter() { 
            if binary_search(&val, target) { 
                return true; 
            }

        }
        
        false
    }
}