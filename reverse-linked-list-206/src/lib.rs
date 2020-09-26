// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution {

}

impl Solution {
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut cur = head;

        while let Some(mut c) = cur.take() {
            let next = c.next.take();
            c.next = prev.take();
            prev = Some(c);
            cur = next;
        }
        prev
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::reverse(None, head)
    }

    pub fn reverse(prev: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut c) = cur {
            let next = c.next;
            c.next = prev;
            return Solution::reverse(Some(c), next);
        }
        prev
    }

    pub fn print_list(head: Option<Box<ListNode>>) {
        let mut cur = head;
        while let Some(c) = cur {
            print!("{} ", c.val);
            cur = c.next;
        }
        println!();
    }
}
