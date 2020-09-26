#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let parentheses = String::from("(({()})[])");
        assert!(Solution::is_valid(parentheses));
    }
}

pub struct Solution {

}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        //let bytes = s.as_bytes();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '}'|')'|']' => if Some(c) != stack.pop() { return false},
                 _  => (),
            }
        }

        stack.is_empty()
    }
}