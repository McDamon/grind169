// https://leetcode.com/problems/invert-binary-tree/

use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = root {
        let left = r.borrow().left.clone();
        let right = r.borrow().right.clone();
        
        r.borrow_mut().left = invert_tree(right);
        r.borrow_mut().right = invert_tree(left);

        return Some(r);
    };
    None
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::grind::invert_binary_tree::{invert_tree, TreeNode};

    #[test]
    fn test_invert_tree_case1() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(4)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(7)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let tree_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_child_l2_3 = Rc::new(RefCell::new(TreeNode::new(6)));
        let tree_child_l2_4 = Rc::new(RefCell::new(TreeNode::new(9)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1.clone());
        tree_root.borrow_mut().right = Some(tree_child_l1_2.clone());

        tree_child_l1_1.borrow_mut().left = Some(tree_child_l2_1);
        tree_child_l1_1.borrow_mut().right = Some(tree_child_l2_2);

        tree_child_l1_2.borrow_mut().left = Some(tree_child_l2_3);
        tree_child_l1_2.borrow_mut().right = Some(tree_child_l2_4);

        let tree_invert_root = Rc::new(RefCell::new(TreeNode::new(4)));

        let tree_invert_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(7)));
        let tree_invert_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_invert_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(9)));
        let tree_invert_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(6)));
        let tree_invert_child_l2_3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_invert_child_l2_4 = Rc::new(RefCell::new(TreeNode::new(1)));

        tree_invert_root.borrow_mut().left = Some(tree_invert_child_l1_1.clone());
        tree_invert_root.borrow_mut().right = Some(tree_invert_child_l1_2.clone());

        tree_invert_child_l1_1.borrow_mut().left = Some(tree_invert_child_l2_1);
        tree_invert_child_l1_1.borrow_mut().right = Some(tree_invert_child_l2_2);

        tree_invert_child_l1_2.borrow_mut().left = Some(tree_invert_child_l2_3);
        tree_invert_child_l1_2.borrow_mut().right = Some(tree_invert_child_l2_4);

        assert_eq!(invert_tree(Some(tree_root)), Some(tree_invert_root));
    }

    #[test]
    fn test_invert_tree_case2() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(3)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1);
        tree_root.borrow_mut().right = Some(tree_child_l1_2);

        let tree_invert_root = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_invert_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_invert_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(1)));

        tree_invert_root.borrow_mut().left = Some(tree_invert_child_l1_1);
        tree_invert_root.borrow_mut().right = Some(tree_invert_child_l1_2);

        assert_eq!(invert_tree(Some(tree_root)), Some(tree_invert_root));
    }

    #[test]
    fn test_invert_tree_case3() {
        assert_eq!(invert_tree(None), None);
    }
}