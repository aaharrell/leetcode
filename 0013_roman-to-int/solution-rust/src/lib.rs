use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    // Solution O(n) time complexity
    // 1. Create a hashmap of roman to int conversions
    // 2. Convert the string input to a char vector
    // 3. Create a mutable int to store the result
    // 4. Loop over the char vector
    //      a. If the vector is empty, return 0
    //      c. Else, loop over the vector beginning at index 0. If the next char doesn't exist or is <=
    //      current, add current to result and proceed to next loop.
    //      d. If the next value > current value, subtract current value and proceed to next loop.
    pub fn roman_to_int(s: String) -> i32 {
        let input_as_chars: Vec<char> = s.chars().collect();
        let roman_int_map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000)
        ]);

        let mut result: i32 = 0;

        if input_as_chars.len() == 0 {
            return result;
        }
        
        for i in 0..input_as_chars.len() {
            let current: i32 = *roman_int_map.get(&input_as_chars[i]).unwrap();
            if (i+1) >= input_as_chars.len(){
                result += current;
            } else {
                let next: i32 = *roman_int_map.get(&input_as_chars[i+1]).unwrap();
                if current >= next {
                    result += current;
                } else {
                    result -= current;
                }
            }
        }
        return result;
    }
}


#[cfg(test)]
mod main_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: String = String::from("III");
        let exp_output: i32 = 3;
        let result: i32 = Solution::roman_to_int(input);

        assert_eq!(exp_output, result);
    }

    #[test]
    fn test2() {
        let input: String = String::from("LVIII");
        let exp_output: i32 = 58;
        let result: i32 = Solution::roman_to_int(input);

        assert_eq!(exp_output, result);
    }

    #[test]
    fn test3() {
        let input: String = String::from("MCMXCIV");
        let exp_output: i32 = 1994;
        let result: i32 = Solution::roman_to_int(input);

        assert_eq!(exp_output, result);
    }
}
