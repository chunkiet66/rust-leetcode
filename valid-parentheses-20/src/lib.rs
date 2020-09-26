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
            if c == '(' {
                stack.push(')');
            } else if c == '{' {
                stack.push('}');
            } else if c == '[' {
                stack.push(']');
            } else {
                match stack.pop() {
                    Some(val) =>
                        if val != c {
                            return false;
                        },
                    None => return false,
                }
            }
        }

        stack.is_empty()
    }
}