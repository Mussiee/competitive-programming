struct MinStack {
    v: Vec<(i32, i32)> 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
       MinStack { 
            v: Vec::new() 
       } 
    }
    
    fn push(&mut self, val: i32) {
        if self.v.len() == 0 { 
            self.v.push((val, val)); 
        } 
        else { 
           if self.v[self.v.len()-1].1 < val { 
                self.v.push((val, self.v[self.v.len()-1].1)); 
           }
           else { 
                self.v.push((val, val)); 
           }
        }
    }
    
    fn pop(&mut self) {
        self.v.pop(); 
    }
    
    fn top(&self) -> i32 {
       self.v[self.v.len()-1].0  
    }
    
    fn get_min(&self) -> i32 {
       self.v[self.v.len()-1].1 
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */