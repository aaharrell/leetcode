pub struct Solution;

impl Solution {
    pub fn merge_lists(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32> {
        // Strategy
        // 1. Create two (2) pointers for each list input
        // 2. Create a new vector to store solution
        // 3. Create loop with len(list1) + len(list2)
        // 4. Compare pointer 1 value to pointer 2 value
        //      a. If pointer 1 value doesn't exist, add pointer 2 value and increment pointer
        //      b. If pointer 2 value doesn't exist, add pointer 1 value and increment pointer
        //      c. If pointer 1 <= pointer 2 value, add pointer 1 value and increment pointer
        //      d. Else add pointer 2 value and increment pointer 2

        let mut solution: Vec<i32> = Vec::new();
        let mut pointer1: usize = 0;
        let mut pointer2: usize = 0;

        for _i in 0..(list1.len() + list2.len()) {
            if pointer1 >= list1.len() {
                solution.push(list2[pointer2]);
                pointer2 += 1;
            } else if pointer2 >= list2.len() {
                solution.push(list1[pointer1]);
                pointer1 += 1;
            } else if list1[pointer1] <= list2[pointer2] {
                solution.push(list1[pointer1]);
                pointer1 += 1;
            } else if list2[pointer2] <= list1[pointer1] {
                solution.push(list2[pointer2]);
                pointer2 += 1;
            }
        }
        return solution;
    }
}

#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input1: Vec<i32> = vec![1, 2, 4];
        let input2: Vec<i32> = vec![1, 3, 4];
        let exp_result: Vec<i32> = vec![1, 1, 2, 3, 4, 4];
        let result = Solution::merge_lists(input1, input2);

        assert_eq!(exp_result, result);
    }

    #[test]
    fn test2() {
        let input1: Vec<i32> = vec![];
        let input2: Vec<i32> = vec![];
        let exp_result: Vec<i32> = vec![];
        let result = Solution::merge_lists(input1, input2);

        assert_eq!(exp_result, result);
    }

    #[test]
    fn test3() {
        let input1: Vec<i32> = vec![0];
        let input2: Vec<i32> = vec![];
        let exp_result: Vec<i32> = vec![0];
        let result = Solution::merge_lists(input1, input2);

        assert_eq!(exp_result, result);
    }

    #[test]
    fn test4() {
        let input1: Vec<i32> = vec![0, 4, 5];
        let input2: Vec<i32> = vec![1, 2, 3];
        let exp_result: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let result = Solution::merge_lists(input1, input2);

        assert_eq!(exp_result, result);
    }
}
