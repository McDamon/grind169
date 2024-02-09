// https://leetcode.com/problems/valid-parentheses/

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Parentheses {
    RoundOpen = b'(',
    RoundClose = b')',
    SquareOpen = b'[',
    SquareClose = b']',
    AngleOpen = b'{',
    AngleClose = b'}',
}

impl TryFrom<u8> for Parentheses {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Parentheses::RoundOpen as u8 => Ok(Parentheses::RoundOpen),
            x if x == Parentheses::RoundClose as u8 => Ok(Parentheses::RoundClose),
            x if x == Parentheses::SquareOpen as u8 => Ok(Parentheses::SquareOpen),
            x if x == Parentheses::SquareClose as u8 => Ok(Parentheses::SquareClose),
            x if x == Parentheses::AngleOpen as u8 => Ok(Parentheses::AngleOpen),
            x if x == Parentheses::AngleClose as u8 => Ok(Parentheses::AngleClose),
            _ => Err(()),
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<Parentheses> = vec![];

        if s.is_empty() {
            return false;
        }

        let first = Parentheses::try_from(s.chars().next().unwrap() as u8).unwrap();

        if first == Parentheses::RoundClose
            || first == Parentheses::SquareClose
            || first == Parentheses::AngleClose
        {
            return false;
        }

        for c in s.chars() {
            match Parentheses::try_from(c as u8) {
                Ok(parentheses) => match parentheses {
                    Parentheses::RoundOpen | Parentheses::SquareOpen | Parentheses::AngleOpen => {
                        stack.push(parentheses);
                        continue;
                    }
                    _ => {
                        if let Some(last_parentheses) = stack.last() {
                            if *last_parentheses == Parentheses::RoundOpen
                                && parentheses == Parentheses::RoundClose
                            {
                                stack.pop();
                                continue;
                            }

                            if *last_parentheses == Parentheses::SquareOpen
                                && parentheses == Parentheses::SquareClose
                            {
                                stack.pop();
                                continue;
                            }

                            if *last_parentheses == Parentheses::AngleOpen
                                && parentheses == Parentheses::AngleClose
                            {
                                stack.pop();
                                continue;
                            }
                        }
                        stack.push(parentheses);
                    }
                },
                Err(_) => panic!("Invalid Parentheses"),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::valid_parentheses::Solution;

    #[test]
    fn test_valid_parentheses_case1() {
        assert_eq!(true, Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_valid_parentheses_case2() {
        assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_valid_parentheses_case3() {
        assert_eq!(false, Solution::is_valid("(]".to_string()));
    }
}
