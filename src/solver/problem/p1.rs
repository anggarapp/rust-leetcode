use std::collections::HashMap;

pub struct P1;

impl P1 {
    pub fn naive_solve(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
    pub fn v1_solve(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
