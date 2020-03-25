//! 1. 两数之和
//! https://leetcode-cn.com/problems/two-sum/

use std::collections::HashMap;
use std::iter::Iterator;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut coll = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let val = target - v;
        let idx = coll.get(&val);
        if idx.is_some() {
            let result = vec![i as i32, *idx.unwrap()];
            return result;
        } else {
            coll.insert(nums[i], i as i32);
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tow_sum() {
        assert_eq!(two_sum(vec![2, 1, 7, 3], 9), vec![2, 0]);
    }
}
