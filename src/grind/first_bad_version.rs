// https://leetcode.com/problems/first-bad-version/

struct Solution {
    bad: i32,
}

impl Solution {
    pub fn is_bad_version(&self, version: i32) -> bool {
        version >= self.bad
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut low = 0;
        let mut high = n;
        self.binary_search(&mut low, &mut high)
    }

    fn binary_search(&self, low: &mut i32, high: &mut i32) -> i32 {
        let mut guess = *high;
        while low <= high {
            let mid = *low + (*high - *low) / 2;
            let is_bad_version = self.is_bad_version(mid);
            //println!("low: {}, high: {}, mid: {}, is_bad_version: {}", low, high, mid, is_bad_version);
            
            // Check if bad version is present at mid
            if is_bad_version {
                // ignore right half
                *high = mid - 1;
                guess = mid;
            }
            else {
                // ignore left half
                *low = mid + 1;
            }
        }
        guess
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::first_bad_version::Solution;

    #[test]
    fn test_is_bad_version_case1() {
        let solution: Solution = Solution { bad: 4 };
        assert_eq!(false, solution.is_bad_version(3));
        assert_eq!(true, solution.is_bad_version(5));
        assert_eq!(true, solution.is_bad_version(4));
    }

    #[test]
    fn test_first_bad_version_case1() {
        let solution: Solution = Solution { bad: 4 };
        assert_eq!(4, solution.first_bad_version(5));
    }

    #[test]
    fn test_first_bad_version_case2() {
        let solution: Solution = Solution { bad: 1 };
        assert_eq!(1, solution.first_bad_version(1));
    }

    #[test]
    fn test_first_bad_version_case3() {
        let solution: Solution = Solution { bad: 2 };
        assert_eq!(2, solution.first_bad_version(2));
    }
    
    #[test]
    fn test_first_bad_version_case4() {
        let solution: Solution = Solution { bad: 1 };
        assert_eq!(1, solution.first_bad_version(4));
    }
    
    #[test]
    fn test_first_bad_version_case5() {
        let solution: Solution = Solution { bad: 2 };
        assert_eq!(2, solution.first_bad_version(4));
    }
}
