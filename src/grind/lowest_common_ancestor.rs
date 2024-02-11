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
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // Storage for paths to our vals
    let mut path_p: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut path_q: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

    // Find a path to each value
    let is_match_p = find_path(&mut path_p, root.clone(), p);
    let is_match_q = find_path(&mut path_q, root, q);

    // If we have a match for both, then compare each path
    if is_match_p || is_match_q {
        let mut path1_ptr = path_p.clone();
        let mut path2_ptr = path_q.clone();

        // now 1 is the largest
        if path1_ptr.len() < path2_ptr.len() {
            std::mem::swap(&mut path1_ptr, &mut path2_ptr);
        }

        let mut mismatch_index = 0;
        for (i, path_tup) in path1_ptr
            .iter()
            .zip(path2_ptr.iter().chain(std::iter::repeat(&None)))
            .enumerate()
        {
            match path_tup {
                (None, None) => (),
                (None, Some(_)) => (),
                (Some(_), None) => {
                    if i != 0 {
                        mismatch_index = i;
                        break;
                    }
                }
                (Some(x), Some(y)) => {
                    if x != y {
                        mismatch_index = i;
                        break;
                    }
                }
            };
        }

        match mismatch_index.cmp(&0) {
            std::cmp::Ordering::Equal => {
                let val = path1_ptr[0].clone().unwrap().borrow().val;
                return Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
            std::cmp::Ordering::Greater => {
                let val = path1_ptr[mismatch_index - 1].clone().unwrap().borrow().val;
                return Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
            std::cmp::Ordering::Less => (),
        }
    }
    None
}

fn find_path(
    path: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    root: Option<Rc<RefCell<TreeNode>>>,
    x: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if let Some(r) = &root {
        let mut is_match_root = false;
        let mut is_match_left = false;
        let mut is_match_right = false;

        path.push(root.clone());

        // If root val is same as our val then we match (node can be a descendant of itself)
        if r.borrow().val == x.as_ref().unwrap().borrow().val {
            is_match_root = true;
        }

        // Check if val is in left subtree
        if let Some(_left) = &r.borrow().left {
            is_match_left = find_path(path, r.borrow().left.clone(), x.clone());
        }

        // Check if val is in right subtree
        if let Some(_right) = &r.borrow().right {
            is_match_right = find_path(path, r.borrow().right.clone(), x);
        }

        if is_match_root || is_match_left || is_match_right {
            return true;
        } else {
            path.pop();
        }
    }
    false
}


#[allow(dead_code)]
pub fn lowest_common_ancestor_non_recursive(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;
    let mut iterator = root;
    let mut result = None;

    while let Some(node) = iterator {
        let mut node = node.borrow_mut();
        let node_val = node.val;
        result = Some(Rc::new(RefCell::new(TreeNode::new(node_val))));
        if node_val > p_val && node_val > q_val {
            iterator = node.left.take();
            continue;
        }

        if node_val < p_val && node_val < q_val {
            iterator = node.right.take();
            continue;
        }
        break;
    }
    result
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::grind::lowest_common_ancestor::lowest_common_ancestor;
    use crate::grind::lowest_common_ancestor::TreeNode;

    fn new_test_tree() -> Rc<RefCell<TreeNode>> {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(6)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(8)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(0)));
        let tree_child_l2_2 = Rc::new(RefCell::new(TreeNode::new(4)));
        let tree_child_l2_3 = Rc::new(RefCell::new(TreeNode::new(7)));
        let tree_child_l2_4 = Rc::new(RefCell::new(TreeNode::new(9)));

        let tree_child_l3_1 = Rc::new(RefCell::new(TreeNode::new(3)));
        let tree_child_l3_2 = Rc::new(RefCell::new(TreeNode::new(5)));

        tree_child_l2_2.borrow_mut().left = Some(tree_child_l3_1);
        tree_child_l2_2.borrow_mut().right = Some(tree_child_l3_2);

        tree_child_l1_2.borrow_mut().left = Some(tree_child_l2_3);
        tree_child_l1_2.borrow_mut().right = Some(tree_child_l2_4);

        tree_child_l1_1.borrow_mut().left = Some(tree_child_l2_1);
        tree_child_l1_1.borrow_mut().right = Some(tree_child_l2_2);

        tree_root.borrow_mut().left = Some(tree_child_l1_1);
        tree_root.borrow_mut().right = Some(tree_child_l1_2);

        tree_root
    }

    #[test]
    fn test_lowest_common_ancestor_case1() {
        let test_tree = new_test_tree();

        let p_node = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(8)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(6)));

        let ans = lowest_common_ancestor(Some(test_tree), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case2() {
        let test_tree = new_test_tree();

        let p_node = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(4)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(2)));

        let ans = lowest_common_ancestor(Some(test_tree), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case3() {
        let test_tree = new_test_tree();

        let p_node = Rc::new(RefCell::new(TreeNode::new(0)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(5)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(2)));

        let ans = lowest_common_ancestor(Some(test_tree), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case4() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(1)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1);

        let p_node = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(1)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(2)));

        let ans = lowest_common_ancestor(Some(tree_root), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case5() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(3)));

        tree_root.borrow_mut().right = Some(tree_child_l1_1);

        let p_node = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(3)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(2)));

        let ans = lowest_common_ancestor(Some(tree_root), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case6() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(2)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(3)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1);
        tree_root.borrow_mut().right = Some(tree_child_l1_2);

        let p_node = Rc::new(RefCell::new(TreeNode::new(3)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(1)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(2)));

        let ans = lowest_common_ancestor(Some(tree_root), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }

    #[test]
    fn test_lowest_common_ancestor_case7() {
        let tree_root = Rc::new(RefCell::new(TreeNode::new(3)));

        let tree_child_l1_1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let tree_child_l1_2 = Rc::new(RefCell::new(TreeNode::new(4)));

        let tree_child_l2_1 = Rc::new(RefCell::new(TreeNode::new(2)));

        tree_root.borrow_mut().left = Some(tree_child_l1_1.clone());
        tree_root.borrow_mut().right = Some(tree_child_l1_2);

        tree_child_l1_1.borrow_mut().right = Some(tree_child_l2_1);

        let p_node = Rc::new(RefCell::new(TreeNode::new(2)));
        let q_node = Rc::new(RefCell::new(TreeNode::new(3)));
        let ans_node = Rc::new(RefCell::new(TreeNode::new(3)));

        let ans = lowest_common_ancestor(Some(tree_root), Some(p_node), Some(q_node));

        assert_eq!(ans, Some(ans_node));
    }
}