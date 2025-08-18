use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let pairs: HashMap<char, char> = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{')
        ]);
        let mut stack: Vec<char> = Vec::new();
        
        for character in s.chars() {
            if let Some(&open) = pairs.get(&character) {
                if stack.pop() != Some(open) {
                    return false;
                }
            } else {
                stack.push(character);
            }
        }
        return stack.is_empty();
    }
}

#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: String = String::from("()");
        let expected_result: bool = true;
        let result = Solution::is_valid(input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test2() {
        let input: String = String::from("()[]{}");
        let expected_result: bool = true;
        let result = Solution::is_valid(input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test3() {
        let input: String = String::from("([])");
        let expected_result: bool = true;
        let result = Solution::is_valid(input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test4() {
        let input: String = String::from("([)]");
        let expected_result: bool = false;
        let result = Solution::is_valid(input);
        assert_eq!(result, expected_result);
    }
}
