// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solutions: HashMap<i32, Vec<i32>> = HashMap::new();

        for (i, num_x) in nums.iter().enumerate() {
            // We know that there is only ONE valid answer
            // If we know x, then we know the value we need for our other number y.
            // In other words, for valid solution y = target - x
            let num_y = target - num_x;

            // Check for a valid soluton
            if let Some(values) = solutions.get(&num_y) {
                for j in values {
                    if i as i32 != *j {
                        // Return our indices
                        let mut result = vec![i as i32, *j];
                        result.sort();
                        return result;
                    }
                }
            } else {
                solutions.insert(*num_x, vec![i as i32]);
                continue;
            }
        }
        Vec::new()
    }

    // Naive O(n^2) solution
    #[allow(dead_code)]
    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num_a) in nums.iter().enumerate() {
            for (j, num_b) in nums.iter().enumerate() {
                if num_a + num_b == target && i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::two_sums::Solution;

    #[test]
    fn test_two_sums_case1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_two_sums_case2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn test_two_sums_case3() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
