//! 28. 实现 strStr()
//! https://leetcode-cn.com/problems/implement-strstr/

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(x) = haystack.find(needle.as_str()) {
            return x as i32;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("hello".into(), "".into()), 0);
    }
}
