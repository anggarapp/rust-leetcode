pub struct P35;
impl P35 {
    pub fn solve_v1(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = (left+right) >> 1;
            if nums[mid] >= target {
                right = mid;
            } else {
                left = mid +1;
            }
        }
        left as i32
    }
    pub fn solve_v2(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(index) => index.try_into().unwrap(),
            Err(index) => index.try_into().unwrap(),
        }    
    }
    pub fn solve_v3(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = (left + right) / 2;
            let mid_val = nums[mid];

            match mid_val.cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return mid as i32,
                Ordering::Greater => right = mid,
            }
        }

        return left as i32;
    }
}