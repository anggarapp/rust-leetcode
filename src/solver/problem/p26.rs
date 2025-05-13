use std::collections::HashSet;

pub struct P26;

impl P26 {
    pub fn solve_v1(nums: &mut Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            set.insert(nums[i]);
        }

        let len = set.len() as i32;
        let mut vec = set.into_iter().collect::<Vec<i32>>();
        vec.sort();
        nums.splice(0..len as usize, vec);
        len
    }
    pub fn solve_v2(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if k == 0 || nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
    pub fn solve_v3(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut idx = 0;
        for i in 1..nums.len() {
            if nums[idx] != nums[i] {
                idx += 1;
                nums[idx] = nums[i];
            }
        }
        (idx + 1) as i32
    }
}
