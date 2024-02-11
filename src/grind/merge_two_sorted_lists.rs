// https://leetcode.com/problems/merge-two-sorted-lists/

use crate::utils::list_node::ListNode;

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (&list1, &list2) {
            (None, None) => return None,
            (None, Some(_)) => return list2,
            (Some(_), None) => return list1,
            _ => (),
        }

        let mut l1_mut = list1;
        let mut l2_mut = list2;

        let mut head: Option<Box<ListNode>> = None;

        let mut tail = &mut head;

        loop {
            let mut l1 = match l1_mut {
                Some(l1) => l1,
                None => {
                    *tail = l2_mut;
                    break;
                }
            };
            let mut l2 = match l2_mut {
                Some(l2) => l2,
                None => {
                    *tail = Some(l1);
                    break;
                }
            };

            if l1.val < l2.val {
                l1_mut = l1.next.take();
                l2_mut = Some(l2);
                *tail = Some(l1);
            } else {
                l2_mut = l2.next.take();
                l1_mut = Some(l1);
                *tail = Some(l2);
            }

            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::{grind::merge_two_sorted_lists::Solution, utils::list_node::ListNode};

    #[test]
    fn test_merge_two_lists_case1() {
        let mut lhs1 = ListNode::new(1);
        let mut lhs2 = ListNode::new(2);
        let lhs3 = ListNode::new(4);

        lhs2.next = Some(Box::new(lhs3));
        lhs1.next = Some(Box::new(lhs2));

        let mut rhs1 = ListNode::new(1);
        let mut rhs2 = ListNode::new(3);
        let rhs3 = ListNode::new(4);

        rhs2.next = Some(Box::new(rhs3));
        rhs1.next = Some(Box::new(rhs2));

        let mut ans1 = ListNode::new(1);
        let mut ans2 = ListNode::new(1);
        let mut ans3 = ListNode::new(2);
        let mut ans4 = ListNode::new(3);
        let mut ans5 = ListNode::new(4);
        let ans6 = ListNode::new(4);

        ans5.next = Some(Box::new(ans6));
        ans4.next = Some(Box::new(ans5));
        ans3.next = Some(Box::new(ans4));
        ans2.next = Some(Box::new(ans3));
        ans1.next = Some(Box::new(ans2));

        assert_eq!(
            Solution::merge_two_lists(Some(Box::new(lhs1)), Some(Box::new(rhs1))),
            Some(Box::new(ans1))
        );
    }

    #[test]
    fn test_merge_two_lists_case2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_merge_two_lists_case3() {
        let rhs1 = ListNode::new(0);

        let ans1 = ListNode::new(0);

        assert_eq!(
            Solution::merge_two_lists(None, Some(Box::new(rhs1))),
            Some(Box::new(ans1))
        );
    }
}
