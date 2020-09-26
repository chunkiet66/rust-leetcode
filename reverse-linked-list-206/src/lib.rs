#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
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

        let mut cur = Solution::reverse_list_recursive(Some(Box::new(e))).unwrap();
        assert_eq!(cur.value(), 1);
        cur = cur.next().unwrap();
        assert_eq!(cur.val, 2);
        cur = cur.next().unwrap();
        assert_eq!(cur.val, 3);
        cur = cur.next().unwrap();
        assert_eq!(cur.val, 4);
        cur = cur.next().unwrap();
        assert_eq!(cur.val, 5);

        assert_eq!(cur.next(), None);
    }
}

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

    pub fn value(&self) -> i32 {
        self.val
    }

    pub fn next(self) -> Option<Box<ListNode>> {
        self.next
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
