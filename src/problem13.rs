//! 13. 罗马数字转整数
//! https://leetcode-cn.com/problems/roman-to-integer/

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars();
        for _c in chars {}
        return 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }
}
