pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_vec: Vec<char> = needle.chars().collect();
        let n = haystack_vec.len();
        let m = needle_vec.len();
        let lps: Vec<usize> = Solution::compute_lps_array(needle);

        // Traversal pointers
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < n {
            // On character match
            if haystack_vec[i] == needle_vec[j] {
                i += 1;
                j += 1;

                // Entire pattern match
                if j == m {
                    return (i-j) as i32;
                }
            }
            // On character mismatch
            else {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }
        // Default; no match
        return -1;
    }

    fn compute_lps_array(pattern: String) -> Vec<usize> {
        let pattern_as_vec: Vec<char> = pattern.chars().collect();
        let n: usize = pattern_as_vec.len();
        let mut lps: Vec<usize> = vec![0usize; n];

        let mut len: usize = 0;
        let mut i: usize = 1;

        while i < n {
            if pattern_as_vec[i] == pattern_as_vec[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        return lps;
    }
}

#[cfg(test)]
mod lps_array_tests {
    use super::Solution;

    #[test]
    fn test1() {
        // "abcaabca"
        // "abcaabcabcaabc"
        // [001234567890]
        let input: String = String::from("ababaa");
        let exp_result: Vec<usize> = vec![0, 0, 1, 2, 3, 1];
        let act_result: Vec<usize> = Solution::compute_lps_array(input);
        assert_eq!(exp_result, act_result);
    }

    #[test]
    fn test2() {
        let input: String = String::from("ababc");
        let exp_result: Vec<usize> = vec![0, 0, 1, 2, 0];
        let act_result: Vec<usize> = Solution::compute_lps_array(input);
        assert_eq!(exp_result, act_result);
    }
}

#[cfg(test)]
mod str_str_tests {
    use super::Solution;

    #[test]
    fn test1() {
        let haystack: String = String::from("sadbutsad");
        let needle: String = String::from("sad");
        let exp_result: i32 = 0;
        let act_result: i32 = Solution::str_str(haystack, needle);
        assert_eq!(exp_result, act_result);

    }

    #[test]
    fn test2() {
        let haystack: String = String::from("leetcode");
        let needle: String = String::from("leeto");
        let exp_result: i32 = -1;
        let act_result: i32 = Solution::str_str(haystack, needle);
        assert_eq!(exp_result, act_result);

    }
}
