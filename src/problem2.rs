//! 2. 两数相加
//! https://leetcode-cn.com/problems/add-two-numbers/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut sum = 0;
    let mut p: Option<Box<ListNode>> = None;
    let mut pp = &mut p;
    loop {
        match (l1, l2) {
            (Some(v1), Some(v2)) => {
                sum += v1.val + v2.val;
                l1 = v1.next;
                l2 = v2.next;
            }
            (Some(v1), None) => {
                sum += v1.val;
                l1 = v1.next;
                l2 = None;
            }
            (None, Some(v2)) => {
                sum += v2.val;
                l1 = None;
                l2 = v2.next;
            }
            (None, None) => break,
        }
        *pp = Some(Box::new(ListNode::new(sum % 10)));
        sum /= 10;
        if let Some(p_inner) = pp {
            pp = &mut p_inner.next
        }
    }

    if sum != 0 {
        *pp = Some(Box::new(ListNode::new(sum)));
    }

    return p;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let a1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        let b1 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        assert_eq!(add_two_numbers(a1, b1), result);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let a1 = Some(Box::new(ListNode::new(0)));

        let b1 = Some(Box::new(ListNode::new(0)));

        let result = Some(Box::new(ListNode::new(0)));

        assert_eq!(add_two_numbers(a1, b1), result);
    }

    #[test]
    fn test_add_two_numbers_3() {
        let a1 = Some(Box::new(ListNode::new(9)));

        let b1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(9))),
        }));

        let result = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(1))),
            })),
        }));

        assert_eq!(add_two_numbers(a1, b1), result);
    }
}
