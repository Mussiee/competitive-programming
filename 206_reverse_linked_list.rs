// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
fn vec_to_list(v: Vec<i32>) -> Option<Box<ListNode>> { 

    //[1,2,3,4] - start from the back and give the v[i] curr node as next node, and move to the start 
    let mut curr = None; 

    for &val in v.iter().rev() { 
        let mut temp = ListNode::new(val); 
        temp.next = curr; 
        curr = Some(Box::new(temp)); 
    }
    curr 
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      //  let test: Vec<i32> = vec![0,1,2,3]; 

//        println!("{:?}", vec_to_list(test));
        let mut dummy = ListNode::new(0); 
        dummy.next = head; 

        let mut curr = dummy.next; 
        let mut prev = None; 
        
        while let Some(mut node) = curr { 
           // println!("{:?}", node.val);  
            let next_node = node.next.take(); 
            node.next = prev; 
            prev = Some(node); 
            curr = next_node;  
        }
        prev
        // we dont need take here, we can just do a while let on the head and do it without take cuz its a local variable
    }
}