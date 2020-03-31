//! 1356. 根据数字二进制下 1 的数目排序
//! https://leetcode-cn.com/problems/sort-integers-by-the-number-of-1-bits/

fn count1(x: i32) -> i32 {
    let mut x = x;
    let mut cnt = 0;
    while x != 0 {
        cnt += x & 1;
        x = x >> 1;
    }
    cnt
}

pub struct Solution;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by(|a, b| count1(*a).cmp(&count1(*b)).then(a.cmp(b)));
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_bits() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::sort_by_bits(arr), vec![0, 1, 2, 4, 8, 3, 5, 6, 7]);

        let arr = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        assert_eq!(
            Solution::sort_by_bits(arr),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );

        let arr = vec![10000, 10000];
        assert_eq!(Solution::sort_by_bits(arr), vec![10000, 10000]);

        let arr = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(
            Solution::sort_by_bits(arr),
            vec![2, 3, 5, 17, 7, 11, 13, 19]
        );

        let arr = vec![10, 100, 1000, 10000];
        assert_eq!(Solution::sort_by_bits(arr), vec![10, 100, 10000, 1000]);
    }
}
