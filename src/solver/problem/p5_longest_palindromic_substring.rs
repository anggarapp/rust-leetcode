use crate::solver::Problem;

impl Problem {
    pub fn solve_longest_palindromic_substring_v1(s: String) -> String {
        let v_chars: Vec<char> = s.chars().collect();
        let v_len = v_chars.len();
        if v_len == 0 {
            return "".to_string();
        }
        let mut start = 0;
        let mut end = 0;
        for i in 0..v_chars.len() {
            let mut left = i;
            let mut right = i;

            while right + 1 < v_len && v_chars[right + 1] == v_chars[left] {
                //expand right if next char same
                right += 1;
            }
            while right + 1 < v_len && left > 0 && v_chars[right + 1] == v_chars[left - 1] {
                // handle palindrome expand left and right
                // if right of right and left of left same
                right += 1;
                left -= 1;
            }

            if right - left > end - start {
                end = right;
                start = left;
            }
        }
        v_chars[start..=end].iter().collect()
    }
}
