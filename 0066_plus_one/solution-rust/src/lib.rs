pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = digits.clone();
        for d in result.iter_mut().rev() {
            *d += 1;
            if *d < 10 {
                return result;     // done, no carry to propagate
            }
            *d = 0;                // keep carrying
        }
        // if we got here, we overflowed (e.g., 9,9,9 -> 0,0,0); prepend 1
        result.insert(0, 1);
        return result;
    }
}

#[cfg(test)]
mod solution_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: Vec<i32> = vec![1, 2, 3];
        let exp_result: Vec<i32> = vec![1, 2, 4];
        let act_result: Vec<i32> = Solution::plus_one(input);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test2() {
        let input: Vec<i32> = vec![4, 3, 2, 1];
        let exp_result: Vec<i32> = vec![4, 3, 2, 2];
        let act_result: Vec<i32> = Solution::plus_one(input);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test3() {
        let input: Vec<i32> = vec![9];
        let exp_result: Vec<i32> = vec![1, 0];
        let act_result: Vec<i32> = Solution::plus_one(input);
        assert_eq!(exp_result, act_result);
    }
}
