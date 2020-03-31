//! 999. 车的可用捕获量
//! https://leetcode-cn.com/problems/available-captures-for-rook

pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        // 首先找白色车的位置
        let mut rook_pos: (i32, i32) = (0, 0);
        'outer: for row_idx in 0..8 {
            for col_idx in 0..8 {
                if board[row_idx][col_idx] == 'R' {
                    rook_pos = (row_idx as i32, col_idx as i32);
                    break 'outer;
                }
            }
        }
        let mut cnt = 0;
        let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (x_p, y_p) in directions {
            let (mut x, mut y) = rook_pos;
            while x > 0 && x < 8 && y > 0 && y < 8 {
                x += x_p;
                y += y_p;
                if x < 0 || x > 7 || y < 0 || y > 7 {
                    break;
                }
                let v = board[x as usize][y as usize];
                if v == 'p' {
                    cnt += 1;
                    break;
                } else if v == 'B' {
                    break;
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_rook_captures() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board), 3);

        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::num_rook_captures(board), 0);
    }
}
