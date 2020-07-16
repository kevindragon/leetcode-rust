//! 9. 回文数
//! https://leetcode-cn.com/problems/palindrome-number/

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut z = 0;
        let mut w = x;
        loop {
            if w == 0 {
                break;
            }
            let y = w % 10;
            z = z * 10 + y;
            w = (w / 10) as i32;
        }
        return x == z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
