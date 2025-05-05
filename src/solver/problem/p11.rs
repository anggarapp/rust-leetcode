use std::cmp;

pub struct P11;

impl P11 {
    pub fn naive_solve(height: Vec<i32>) -> i32 {
        let arr_len = height.len();
        let mut max_area = 0;
        for tall in 0..=arr_len {
            for width in (tall + 1)..arr_len {
                let area = cmp::min(height[tall], height[width]) * (width as i32 - tall as i32);
                max_area = cmp::max(area, max_area);
            }
        }
        max_area
    }

    pub fn two_pointer_solve(height: Vec<i32>) -> i32 {
        let mut left_pointer = 0;
        let mut right_pointer = (height.len() - 1) as i32;
        let mut result = 0;

        while left_pointer < right_pointer {
            let area = cmp::min(
                height[left_pointer as usize],
                height[right_pointer as usize],
            ) * (right_pointer - left_pointer);
            result = cmp::max(result, area);

            if height[left_pointer as usize] < height[right_pointer as usize] {
                left_pointer += 1;
            } else {
                right_pointer -= 1;
            }
        }
        result
    }
}
