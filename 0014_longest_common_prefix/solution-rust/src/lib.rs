pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: String = String::new();
        if strs.len() == 0 {
            return prefix;
        }

        for i in 0..strs[0].len() {
            let current_letter: char = strs[0].chars().nth(i).unwrap();
            for j in 0..strs.len() {
                if (strs[j].chars().count() <= i) ||
                    (strs[j].chars().nth(i).unwrap() != current_letter) {
                        return prefix;
                }
            }
            prefix.push(current_letter);
        }

        return prefix;
    }
}

#[cfg(test)]
mod main_test {
    use super::Solution;

    #[test]
    fn test1() {
        let input: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()];
        let expect: String = String::from("fl");
        let result: String = Solution::longest_common_prefix(input);
        assert_eq!(expect, result);
    }

    #[test]
    fn test2() {
        let input: Vec<String> = vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()];
        let expect: String = String::from("");
        let result: String = Solution::longest_common_prefix(input);
        assert_eq!(expect, result);
    }
}
