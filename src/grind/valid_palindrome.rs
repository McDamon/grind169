// https://leetcode.com/problems/valid-palindrome/

struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_alphanum = s
            .to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        let s_reversed = s_alphanum.chars().rev().collect::<String>();

        s_alphanum == s_reversed
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::valid_palindrome::Solution;

    #[test]
    fn test_is_palindrome_case1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test_is_palindrome_case2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn test_is_palindrome_case3() {
        assert!(Solution::is_palindrome(" ".to_string()));
    }
}
