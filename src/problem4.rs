//! 4. 寻找两个正序数组的中位数
//! https://leetcode-cn.com/problems/median-of-two-sorted-arrays/

//! 关键点：
//! 1. 在最短的数组上面进行位置的计算，可以让算法的时间复杂度为：O(log(min(m, n)))
//! 2. 中位数的位置肯定在 (m + n + 1) / 2 的位置的左边最大值与右边最小值的平均
//! 3. 第一个数组的位置用i表示，那么第二个数组的位置肯定是：((m + n + 1) / 2) - i
//! 4. 使用二分查找在最短的数组上查找

use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut arr1 = &nums1;
        let mut arr2 = &nums2;
        if nums1.len() > nums2.len() {
            arr1 = &nums2;
            arr2 = &nums1;
        }

        let m = arr1.len();
        let n = arr2.len();
        let total_left = (m + n + 1) / 2;

        let mut left = 0;
        let mut right = m;
        while left < right {
            let i = left + (right - left + 1) / 2;
            let j = total_left - i;
            if arr1[i-1] > arr2[j] {
                right = i - 1;
            } else {
                left = i;
            }
        }

        let i = left;
        let j = total_left - i;
        let arr1_left_max = if i == 0 { std::i32::MIN } else { arr1[i-1] };
        let arr1_right_min = if i == m { std::i32::MAX } else { arr1[i] };
        let arr2_left_max = if j == 0 { std::i32::MIN } else { arr2[j-1] };
        let arr2_right_min = if j == n { std::i32::MAX } else { arr2[j] };

        if (m + n) % 2 == 0 {
            let left_max = max(arr1_left_max, arr2_left_max) as f64;
            let right_min = min(arr1_right_min, arr2_right_min) as f64;
            (left_max + right_min) / 2.0
        } else {
            max(arr1_left_max, arr2_left_max) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums1 = vec![1,3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test2() {
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test3() {
        let nums1 = vec![0,0];
        let nums2 = vec![0,0];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 0.0);
    }

    #[test]
    fn test4() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test5() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test6() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 4.5);
    }
}