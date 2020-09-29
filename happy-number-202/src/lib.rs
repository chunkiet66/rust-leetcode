#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::is_happy(19));
    }
}

use std::collections::HashSet;

pub struct Solution {

}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(n);

        let mut total = n;
        let mut n = n;
        while total != 1 {
            total = 0;
            while n != 0 {
                let cur = n%10;
                n = n/10;
                total = total + cur*cur;
            }
            if set.contains(&total) {
                return false;
            }
            set.insert(total);
            n = total;
        }

        true
    }
}
