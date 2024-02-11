// https://leetcode.com/problems/valid-anagram/

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut svec = s.chars().collect::<Vec<_>>();
        let mut tvec = t.chars().collect::<Vec<_>>();
        svec.sort_unstable();
        tvec.sort_unstable();
        svec.into_iter().collect::<String>() == tvec.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::valid_anagram::Solution;

    #[test]
    fn test_is_anagram_case1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn test_is_anagram_case2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
