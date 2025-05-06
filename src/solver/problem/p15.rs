pub struct P15;
use std::cmp::Ordering;

impl P15 {
    pub fn fail_solve_v1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let nums_length = nums.len();

        for i in 0..nums_length - 2 {
            for j in i + 1..nums_length - 1 {
                for k in j + 1..nums_length {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        // results.push(vec![i as i32, j as i32, k as i32]);
                        results.push(vec![nums[i], nums[j], nums[k]]);
                    }
                }
            }
        }

        results
    }

    pub fn solve_v1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_lists = nums;
        num_lists.sort();
        let nums_length = num_lists.len();

        let mut results = Vec::new();

        for i in 0..nums_length - 2 {
            if num_lists[i] > 0 {
                break;
            }
            if i != 0 && num_lists[i] == num_lists[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums_length - 1;
            while j < k {
                let x = num_lists[i] + num_lists[j] + num_lists[k];

                if x < 0 {
                    j += 1;
                } else if x > 0 {
                    k -= 1;
                } else {
                    results.push(vec![num_lists[i], num_lists[j], num_lists[k]]);
                    j += 1;
                    k -= 1;

                    while j < k && num_lists[j] == num_lists[j - 1] {
                        j += 1;
                    }
                    while j < k && num_lists[k] == num_lists[k + 1] {
                        k -= 1;
                    }
                }
            }
        }
        results
    }

    pub fn solve_v2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let num_length = nums.len();
        let mut results = vec![];
        let mut index = 0;

        while index < num_length - 2 && nums[index] <= 0 {
            let mut left = index + 1;
            let mut right = num_length - 1;

            while left < right {
                match (nums[index] + nums[left] + nums[right]).cmp(&0) {
                    Ordering::Less => {
                        left += 1;
                    }
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Equal => {
                        results.push(vec![nums[index], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < num_length && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while right > 0 && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                }
            }
            index += 1;
            while index < num_length - 2 && nums[index] == nums[index - 1] {
                index += 1;
            }
        }
        results
    }
}
