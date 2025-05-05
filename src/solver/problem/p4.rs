pub struct P4;

impl P4 {
    pub fn solve(nums_1: Vec<i32>, nums_2: Vec<i32>) -> f64 {
        let mut merged_nums = vec![];
        merged_nums.extend(nums_1);
        merged_nums.extend(nums_2);
        merged_nums.sort();
        let vec_len = merged_nums.len() as i32;
        let vec_med_len = vec_len / 2;
        if vec_len % 2 == 0 {
            if merged_nums[(vec_med_len - 1) as usize]
                == merged_nums[((vec_med_len - 1) + 1) as usize]
            {
                return merged_nums[(vec_med_len - 1) as usize] as f64;
            }
            let first_half = merged_nums[(vec_med_len - 1) as usize] as f64;
            let end_half = merged_nums[vec_med_len as usize] as f64;
            return (first_half + end_half) / 2.0;
        }
        merged_nums[(vec_med_len) as usize] as f64
    }
}
