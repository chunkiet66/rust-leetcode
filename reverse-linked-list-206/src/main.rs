use reverse_linked_list::*;

fn main () {
    println!("testing reverse linked list");
    let a = ListNode::new(1);
        let b = ListNode {
            val: 2,
            next: Some(Box::new(a)),
        };
        let c = ListNode {
            val: 3,
            next: Some(Box::new(b)),
        };
        let d = ListNode {
            val: 4,
            next: Some(Box::new(c)),
        };
        let e = ListNode {
            val: 5,
            next: Some(Box::new(d)),
        };

        Solution::print_list(Some(Box::new(e.clone())));
        let c = Solution::reverse_list_recursive(Some(Box::new(e.clone())));
        Solution::print_list(c);
}
