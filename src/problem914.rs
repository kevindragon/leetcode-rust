//! 914. 卡牌分组
//! https://leetcode-cn.com/problems/x-of-a-kind-in-a-deck-of-cards/

use std::collections::HashMap;

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y;
    }
    gcd(y % x, x)
}

pub struct Solution;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        if deck.len() <= 1 {
            return false;
        }

        let mut counts: [i32; 10000] = [0; 10000];
        for x in deck {
            counts[x as usize] += 1;
        }

        let mut g = counts[0];
        for i in 1..counts.len() {
            g = gcd(g, counts[i]);
        }
        g >= 2
    }

    pub fn has_groups_size_x2(deck: Vec<i32>) -> bool {
        if deck.len() <= 1 {
            return false;
        }

        let coll = deck.iter().fold(HashMap::new(), |mut acc, x| {
            if let Some(v) = acc.get_mut(&x) {
                *v += 1;
            } else {
                acc.insert(x, 1);
            }
            acc
        });

        let mut g = -1;
        for x in coll.values() {
            if g == -1 {
                g = *x;
            } else {
                g = gcd(g, *x);
            }
        }
        g >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_groups_size_x() {
        assert_eq!(
            Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]),
            true
        );
        assert_eq!(
            Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]),
            false
        );
        assert_eq!(Solution::has_groups_size_x(vec![1]), false);
        assert_eq!(Solution::has_groups_size_x(vec![1, 1]), true);
        assert_eq!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]), true);
    }

    #[test]
    fn test_has_groups_size_x2() {
        assert_eq!(
            Solution::has_groups_size_x2(vec![1, 2, 3, 4, 4, 3, 2, 1]),
            true
        );
        assert_eq!(
            Solution::has_groups_size_x2(vec![1, 1, 1, 2, 2, 2, 3, 3]),
            false
        );
        assert_eq!(Solution::has_groups_size_x2(vec![1]), false);
        assert_eq!(Solution::has_groups_size_x2(vec![1, 1]), true);
        assert_eq!(Solution::has_groups_size_x2(vec![1, 1, 2, 2, 2, 2]), true);
    }
}
