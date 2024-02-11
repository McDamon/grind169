// https://leetcode.com/problems/two-sum/

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {  
        let mut mutable_ransom_note = ransom_note;
        let magazine_vec: Vec<char> = magazine.chars().into_iter().collect();
        
        for letter in magazine_vec {
            if let Some(index) = mutable_ransom_note.find(letter) {
                mutable_ransom_note.remove(index);
            }
        }
        mutable_ransom_note.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::ransom_note::Solution;

    #[test]
    fn test_ransom_note_case1() {
        assert_eq!(false, Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn test_ransom_note_case2() {
        assert_eq!(false, Solution::can_construct("aa".to_string(), "bb".to_string()));
    }

    #[test]
    fn test_ransom_note_case3() {
        assert_eq!(true, Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}
