//! 151. 翻转字符串里的单词
//! https://leetcode-cn.com/problems/reverse-words-in-a-string/

pub struct Solution;

impl Solution {
    pub fn reverse_words2(s: String) -> String {
        let mut stack = vec![];
        let mut stash = String::new();
        for c in s.chars() {
            if c == ' ' {
                if !stash.is_empty() {
                    stack.push(stash);
                    stash = String::new();
                }
            } else {
                stash.push(c);
            }
        }
        if !stash.is_empty() {
            stack.push(stash);
        }
        stack.reverse();
        stack.join(" ")
    }

    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".into()),
            String::from("blue is sky the")
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".into()),
            String::from("world! hello")
        );
        assert_eq!(
            Solution::reverse_words("a good   example".into()),
            String::from("example good a")
        );
    }
}
