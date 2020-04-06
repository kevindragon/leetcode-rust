//! 72. 编辑距离
// https://leetcode-cn.com/problems/edit-distance/

pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        let mut table: Vec<Vec<i32>> = vec![vec![0; len1 + 1]; len2 + 1];
        // first row
        for i in 0..=len1 {
            table[0][i] = i as i32;
        }
        // first column
        for i in 0..=len2 {
            table[i][0] = i as i32;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let m1 = (table[j - 1][i] + 1).min(table[j][i - 1] + 1);
                if chars1[i - 1] == chars2[j - 1] {
                    table[j][i] = m1.min(table[j - 1][i - 1]);
                } else {
                    table[j][i] = m1.min(table[j - 1][i - 1] + 1);
                }
            }
        }

        // for c in word1.chars() {
        //     print!("{:4}", c);
        // }
        // println!("");
        // for i in 0..=len2 {
        //     if i == 0 {
        //         print!("{:4}", '#');
        //     } else {
        //         print!("{:4}", chars2[i - 1]);
        //     }
        //     for j in 0..=len1 {
        //         print!("{:<4}", table[i][j]);
        //     }
        //     println!("");
        // }

        table[len2][len1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
        assert_eq!(
            Solution::min_distance("intention".into(), "execution".into()),
            5
        );
        assert_eq!(
            Solution::min_distance("zoologicoarchaeologist".into(), "zoogeologist".into()),
            10
        );
    }
}
