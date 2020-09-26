#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        assert_eq!(Solution::find_kth_largest(nums, k), 5);

        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), 4);
    }
}

use std::collections::BinaryHeap;

pub struct Solution {
}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for num in nums.into_iter() {
            heap.push(num);
        }
        for _ in 1..k {
            heap.pop();
        }
        heap.pop().unwrap()
    }
}
