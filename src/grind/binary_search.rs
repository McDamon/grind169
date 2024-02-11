// https://leetcode.com/problems/binary-search/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_inner(&nums, target, 0, nums.len() as i32)
    }

    fn search_inner(nums: &Vec<i32>, target: i32, begin: i32, end: i32) -> i32 {
        if begin >= end {
            return -1;
        }

        // Find midpoint of index of nums
        let midpoint = (begin + end) / 2;

        match target.cmp(&nums[midpoint as usize]) {
            Ordering::Less => Self::search_inner(nums, target, begin, midpoint),
            Ordering::Greater => Self::search_inner(nums, target, midpoint + 1, end),
            Ordering::Equal => midpoint,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::binary_search::Solution;

    #[test]
    fn test_search_case1() {
        assert_eq!(Solution::search([-1, 0, 3, 5, 9, 12].to_vec(), 9), 4);
    }

    #[test]
    fn test_search_case2() {
        assert_eq!(Solution::search([-1, 0, 3, 5, 9, 12].to_vec(), 2), -1);
    }
}
