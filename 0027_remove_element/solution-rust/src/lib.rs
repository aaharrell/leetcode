pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Strategy
        // 1. Create a counter starting at 0
        // 2. Loop over every element. If != val, change counter position's value and increment
        //    counter
        // 3. Return counter

        let mut counter: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[counter] = nums[i];
                counter += 1;
            }
        }
        return counter as i32;
    }
}


#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let mut nums: Vec<i32> = vec![3, 2, 2, 3];
        let val: i32 = 3;
        let exp_output: i32 = 2;
        let result: i32 = Solution::remove_element(&mut nums, val);
        assert_eq!(exp_output, result);
        assert_eq!(nums, vec![2, 2, 2, 3]);
    }

    #[test]
    fn test2() {
        let mut nums: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val: i32 = 2;
        let exp_output: i32 = 5;
        let result: i32 = Solution::remove_element(&mut nums, val);
        assert_eq!(exp_output, result);
        assert_eq!(nums, vec![0, 1, 3, 0, 4, 0, 4, 2]);
    }
}
