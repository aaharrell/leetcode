pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let mut reverse: i32 = 0;
        let mut x_copy: i32 = x;

        loop {
            reverse = (reverse * 10) + (x_copy % 10);
            x_copy /= 10;
            
            if x_copy <= 0 {
                break
            }
        }

        return reverse == x;
    }
}


#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: i32 = 121;
        let expected_result: bool = true;
        let result: bool = Solution::is_palindrome(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test2() {
        let input: i32 = -121;
        let expected_result: bool = false;
        let result: bool = Solution::is_palindrome(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test3() {
        let input: i32 = 10;
        let expected_result: bool = false;
        let result: bool = Solution::is_palindrome(input);

        assert_eq!(result, expected_result);
    }
}
