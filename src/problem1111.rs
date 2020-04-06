//! 1111. 有效括号的嵌套深度
//! https://leetcode-cn.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/

pub struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut acc = vec![];
        seq.chars().fold(0, |idx, c| {
            if c == '(' {
                acc.push(idx & 1);
            } else {
                acc.push((idx + 1) & 1);
            }
            idx + 1
        });
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth_after_split() {
        assert_eq!(
            Solution::max_depth_after_split("(()())".into()),
            vec![0, 1, 1, 1, 1, 0]
        );

        assert_eq!(
            Solution::max_depth_after_split("()(())()".into()),
            vec![0, 0, 0, 1, 1, 0, 0, 0]
        );
    }
}
