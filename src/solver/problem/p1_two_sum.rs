use crate::solver::Problem;
use std::collections::HashMap;

impl Problem {
    pub fn solve_two_sum_v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_one = nums.clone();
        let nums_two = nums;
        for (index_one, number_one) in nums_one.iter().enumerate() {
            for (index_two, number_two) in nums_two.iter().enumerate() {
                if (*number_one + *number_two) == target {
                    return vec![index_one as i32, index_two as i32];
                }
            }
        }
        vec![]
    }
    pub fn solve_two_sum_vx(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            match num_map.get(num) {
                Some(&index) => return vec![index, i as i32],
                None => num_map.insert(target - num, i as i32),
            };
        }
        vec![]
    }
}
