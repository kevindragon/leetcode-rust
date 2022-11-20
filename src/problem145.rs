//! 145. 二叉树的后序遍历
// https://leetcode.cn/problems/binary-tree-postorder-traversal/

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn postorder_dfs(node: &Option<Rc<RefCell<TreeNode>>>, coll: &mut Vec<i32>) {
            if let Some(x) = node {
                postorder_dfs(&x.borrow().left, coll);
                postorder_dfs(&x.borrow().right, coll);
                coll.push(x.borrow().val);
            }
        }
        let mut coll = vec![];
        postorder_dfs(&root, &mut coll);
        coll
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let root: Option<_> = None;
        let coll = Solution::postorder_traversal(root);
        let expected: Vec<i32> = vec![];
        assert_eq!(expected, coll);
    }

    #[test]
    fn test_case1() {
        let root = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
            right: Some(Rc::new(RefCell::new(TreeNode { val: 4, left: None, right: None }))),
        };
        let coll = Solution::postorder_traversal(Some(Rc::new(RefCell::new(root))));
        let expected = vec![2, 4, 3];
        assert_eq!(expected, coll);
    }

    #[test]
    fn test_case2() {
        let root = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode { val: 5, left: None, right: None }))),
                right: None
            }))),
        };
        let coll = Solution::postorder_traversal(Some(Rc::new(RefCell::new(root))));
        let expected = vec![2, 5, 6, 3];
        assert_eq!(expected, coll);
    }
}