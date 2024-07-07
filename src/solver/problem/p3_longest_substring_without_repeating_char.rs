use crate::solver::Problem;

impl Problem {
    pub fn solve_longest_substring_without_repeating_char_v1(string: String) -> i32 {
        use std::collections::HashSet;

        let mut count = 0i32;

        for (i, ic) in string.chars().enumerate() {
            let mut chars = HashSet::new();
            chars.insert(ic);
            for jc in string.chars().skip(i + 1) {
                if !chars.insert(jc) {
                    break;
                }
            }

            let len = chars.len() as i32;
            if len > count {
                count = len;
            }
        }

        count
    }
    pub fn solve_longest_substring_without_repeating_char_vx(string: String) -> i32 {
        let seq: Vec<char> = string.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[idx] == seq[end] {
                    start = idx + 1; //sliding start to right
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1 // sliding end to right
        }
        max as i32
    }
}
