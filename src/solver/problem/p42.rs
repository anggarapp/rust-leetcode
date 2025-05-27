pub struct P42;

impl P42 {
    pub fn solve_v1(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left: Vec<i32> = vec![0; n];
        let mut right: Vec<i32> = vec![0; n];
        left[0] = height[0];
        right[n - 1] = height[n - 1];

        for i in 1..n {
            left[i] = std::cmp::max(left[i - 1], height[i]);
            right[n - i - 1] = std::cmp::max(right[n - i], height[n - i - 1]);
        }

        let mut results = 0;

        for i in 0..n {
            results += std::cmp::min(left[i], right[i]) - height[i];
        }
        results
    }
    pub fn solve_v2(height: Vec<i32>) -> i32 {
        use core::slice::Iter;

        fn trap_water(map: &[i32]) -> i32 {
            let mut result_water: i32 = 0;
            let mut current_water: i32 = 0;
            let mut height: i32 = 0;
            let mut last_peak: Option<Iter<i32>> = None;

            let mut iter = map.iter();
            loop {
                let i: i32;
                match iter.next() {
                    Some(value) => i = *value,
                    None => break,
                }
                if i >= height {
                    result_water += current_water;
                    current_water = 0;
                    last_peak = Some(iter.clone());
                    height = i;
                    continue;
                }
                current_water += height - i;
            }
            let peak: Iter<i32>;
            match last_peak {
                Some(value) => peak = value,
                None => return 0,
            }
            height = 0;
            for i in peak.rev() {
                if *i >= height {
                    height = *i;
                    continue;
                }
                result_water += height - *i;
            }

            return result_water;
        }

        trap_water(height.as_slice())
    }
}
