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
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode::new(0)); 
        let mut ans = &mut dummy;  

        while let Some(ref node1) = list1 && let Some(ref node2) = list2 { 
            if node1.val <= node2.val { 
                ans.next = Some(Box::new(ListNode::new(node1.val))); 
                list1 = list1.unwrap().next;  
                ans = ans.next.as_mut().unwrap();  
            }else { 
                ans.next = Some(Box::new(ListNode::new(node2.val))); 
                list2 = list2.unwrap().next;  
                ans = ans.next.as_mut().unwrap();  
            }
            //println!("{:?}", .val); 
        }
        while let Some(node1) = list1 { 
            ans.next =  Some(Box::new(ListNode::new(node1.val))); 
            list1 = node1.next; 
            ans = ans.next.as_mut().unwrap();  
        }

        while let Some(node2) = list2 { 
            ans.next = Some(Box::new(ListNode::new(node2.val))); 
            list2 = node2.next;  
            ans = ans.next.as_mut().unwrap();  
        }

        
        dummy.next
    }
}