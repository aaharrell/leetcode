use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // --- Naive solution, O(n^2) time complexity ---
        // for i in 0..(nums.len()) {
        //     for j in (i+1)..(nums.len()) {
        //         if (nums[i] + nums[j]) == target {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }
        // return Vec::new()

        // --- Better solution, O(n) time complexity ---
        // 1. Loop over contents of the vector
        // 2. Map structure will be "needed value: index"
        // 3. At each index, calculate: needed = target-current
        // 4. If needed in map, done. Return [needed_value, current_index]
        // 5. If not in map, add "needed_value: current index"
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let needed: i32 = target - nums[i];
            if let Some(&j) = map.get(&needed) {
                return vec![j as i32, i as i32]
            } else {
                map.insert(nums[i], i);
            }
        }
        return Vec::new();
    }
}


#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let result: Vec<i32> = Solution::two_sum(input, target);

        assert_eq!(result, vec![0,1])
    }

    #[test]
    fn test2() {
        let input: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;
        let result: Vec<i32> = Solution::two_sum(input, target);

        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn test3() {
        let input: Vec<i32> = vec![3,3];
        let target: i32 = 6;
        let result: Vec<i32> = Solution::two_sum(input, target);

        assert_eq!(result, vec![0, 1])
    }
}
