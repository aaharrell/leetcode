pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<String> = s
            .trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        if words.len() == 0 {
            return 0;
        } else {
            let last_word: &String = &words[words.len() - 1];
            return last_word.chars().count() as i32;
        }
    }
}

#[cfg(test)]
mod solution_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let input: String = String::from("Hello World");
        let exp_output: i32 = 5;
        let act_output: i32 = Solution::length_of_last_word(input);
        assert_eq!(exp_output, act_output);
    }

    #[test]
    fn test2() {
        let input: String = String::from("    fly me    to    the moon  ");
        let exp_output: i32 = 4;
        let act_output: i32 = Solution::length_of_last_word(input);
        assert_eq!(exp_output, act_output);
    }

    #[test]
    fn test3() {
        let input: String = String::from("luffy is still joyboy");
        let exp_output: i32 = 6;
        let act_output: i32 = Solution::length_of_last_word(input);
        assert_eq!(exp_output, act_output);
    }
}
