//! 面试题62. 圆圈中最后剩下的数字
//! https://leetcode-cn.com/problems/yuan-quan-zhong-zui-hou-sheng-xia-de-shu-zi-lcof/

pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut p = 0;
        for i in 2..=n {
            p = (p + m) % i;
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_remaining() {
        assert_eq!(Solution::last_remaining(5, 3), 3);
        assert_eq!(Solution::last_remaining(10, 17), 2);
        assert_eq!(Solution::last_remaining(10, 2), 4);
    }
}
