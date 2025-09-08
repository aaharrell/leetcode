pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Simple binary search
        let mut low: usize = 0;
        let mut high: usize = nums.len();

        while low <= high {
            let mid: usize = low + (high - low) / 2;
            if target < nums[0] {
                return 0;
            }
            if target > nums[nums.len() - 1] {
                return nums.len() as i32;
            }
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        return low as i32;
    }
}

#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target: i32 = 5;
        let exp_result: i32 = 2;
        let act_result: i32 = Solution::search_insert(nums, target);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test2() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target: i32 = 2;
        let exp_result: i32 = 1;
        let act_result: i32 = Solution::search_insert(nums, target);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test3() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target: i32 = 7;
        let exp_result: i32 = 4;
        let act_result: i32 = Solution::search_insert(nums, target);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test4() {
        let nums: Vec<i32> = vec![1, 3, 5, 6];
        let target: i32 = 0;
        let exp_result: i32 = 0;
        let act_result: i32 = Solution::search_insert(nums, target);
        assert_eq!(exp_result, act_result);
    }
}
