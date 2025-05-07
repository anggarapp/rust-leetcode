pub struct P16;

impl P16 {
    pub fn solve_v1(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut results = 1 << 30;
        for (i, v) in nums.iter().enumerate() {
            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let t = v + nums[j] + nums[k];
                if t == target {
                    return t;
                }
                if (t - target).abs() < (results - target).abs() {
                    results = t;
                }
                if t > target {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        results
    }

    pub fn solve_v2(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let mut closest_sum = nums[0] + nums[1] + nums[2];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];
                if (current_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = current_sum;
                }

                if current_sum < target {
                    left += 1
                } else if current_sum > target {
                    right -= 1;
                } else {
                    return current_sum;
                }
            }
        }
        closest_sum
    }
}
