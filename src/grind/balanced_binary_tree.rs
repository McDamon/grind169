// https://leetcode.com/problems/balanced-binary-tree/

use std::{cell::RefCell, cmp, rc::Rc};

use crate::utils::tree_node::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();

            // First condition: absolute difference between heights of left and right subtrees
            // should less than or equal to 1
            let subtree_height_left = Self::subtree_height(node.left.clone());

            let subtree_height_right = Self::subtree_height(node.right.clone());

            let height_diff = (subtree_height_left - subtree_height_right).abs();

            // Second condition: each left and right subtree should be balanced
            let balanced_left = Self::is_balanced(node.left.clone());

            let balanced_right = Self::is_balanced(node.right.clone());

            return height_diff <= 1 && balanced_left && balanced_right;
        }
        true
    }

    fn subtree_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            let mut height_left = 0;
            let mut height_right = 0;

            if let Some(left) = &node.left {
                height_left = Self::subtree_height(Some(left.clone()));
            }

            if let Some(right) = &node.right {
                height_right = Self::subtree_height(Some(right.clone()));
            }

            return cmp::max(height_left, height_right) + 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::grind::balanced_binary_tree::Solution;
    use crate::grind::balanced_binary_tree::TreeNode;

    #[test]
    fn test_is_balanced_case1() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(3)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(9)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(20)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(15)));
        let tree_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(7)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1);
        tree_root.borrow_mut().right = Some(tree_child_l1_2.clone());

        tree_child_l1_2.borrow_mut().left = Some(tree_child_l2_1);
        tree_child_l1_2.borrow_mut().right = Some(tree_child_l2_2);

        assert!(Solution::is_balanced(Some(tree_root)));
    }

    #[test]
    fn test_is_balanced_case2() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(1)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(3)));

        let tree_child_l3_1 = Rc::new(RefCell::new(TreeNode::new(4)));
        let tree_child_l3_2 = Rc::new(RefCell::new(TreeNode::new(4)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1.clone());
        tree_root.borrow_mut().right = Some(tree_child_l1_2);

        tree_child_l1_1.borrow_mut().left = Some(tree_child_l2_1.clone());
        tree_child_l1_1.borrow_mut().right = Some(tree_child_l2_2);

        tree_child_l2_1.borrow_mut().left = Some(tree_child_l3_1);
        tree_child_l2_1.borrow_mut().right = Some(tree_child_l3_2);

        assert!(!Solution::is_balanced(Some(tree_root)));
    }

    #[test]
    fn test_is_balanced_case3() {
        assert!(Solution::is_balanced(None));
    }

    #[test]
    fn test_is_balanced_case4() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(1)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(3)));

        let tree_child_l3_1 = Rc::new(RefCell::new(TreeNode::new(4)));
        let tree_child_l3_2 = Rc::new(RefCell::new(TreeNode::new(4)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1.clone());
        tree_root.borrow_mut().right = Some(tree_child_l1_2.clone());

        tree_child_l1_1.borrow_mut().left = Some(tree_child_l2_1.clone());
        tree_child_l1_2.borrow_mut().right = Some(tree_child_l2_2.clone());

        tree_child_l2_1.borrow_mut().left = Some(tree_child_l3_1);
        tree_child_l2_2.borrow_mut().right = Some(tree_child_l3_2);

        assert!(!Solution::is_balanced(Some(tree_root)));
    }
}
