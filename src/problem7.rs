//! 7. 整数反转
//! https://leetcode-cn.com/problems/reverse-integer

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if (rev > i32::max_value() / 10) || (rev == i32::max_value() / 10 && pop > 7) {
                return 0;
            }
            if (rev < i32::min_value() / 10) || (rev == i32::min_value() / 10 && pop < -8) {
                return 0;
            }
            rev = rev * 10 + pop;
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
