//! 3. 无重复字符的最长子串
//! https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut sub = String::new();
        for c in s.chars() {
            if let Some(idx) = sub.find(c) {
                let (_, b) = sub.split_at(idx + 1);
                sub = String::from(b);
                // 这样使用slice再转String慢一些
                // sub = sub[idx + 1..].to_string();
            }
            sub.push(c);
            let len = sub.chars().count();
            if len > max {
                max = len;
            }
        }

        return max as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
