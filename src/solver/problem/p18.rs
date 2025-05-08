pub struct P18;

impl P18 {
    pub fn solve_v1(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let nums_len = nums.len();
        let mut answers = Vec::new();
        if nums_len < 4 {
            return answers;
        }
        let mut nums = nums;
        let target = target as i64;
        nums.sort();

        for i in 0..nums_len - 3 {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..nums_len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut k = j + 1;
                let mut l = nums_len - 1;

                while k < l {
                    let x = nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;
                    if x < target {
                        k += 1
                    } else if x > target {
                        l -= 1
                    } else {
                        answers.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        k += 1;
                        l -= 1;
                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                        while k < l && nums[l] == nums[l + 1] {
                            l -= 1;
                        }
                    }
                }
            }
        }
        answers
    }
    pub fn solve_v2(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut vv = Vec::new();
        let mut nums = nums;
        nums.sort();
        if nums.len() < 4 {
            return vv;
        }

        for i in 0..(nums.len() - 3) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if target / 4 < nums[i] {
                break;
            }
            for j in (i + 1)..(nums.len() - 2) {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let mut l = j + 1;
                let mut r = nums.len() - 1;
                while l < r {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[l] as i64 + nums[r] as i64;
                    if sum > i32::MAX as i64 {
                        r -= 1;
                    } else if sum < i32::MIN as i64 {
                        l += 1;
                    } else {
                        let sum = sum as i32;
                        if sum < target {
                            l += 1;
                        } else if sum > target {
                            r -= 1;
                        } else {
                            vv.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            l += 1;
                            while nums[l] == nums[l - 1] && l < r {
                                l += 1;
                            }
                        }
                    }
                }
            }
        }
        return vv;
    }
    pub fn solve_v3(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, HashSet};
        let n = nums.len();

        if n < 4 {
            return vec![];
        }

        // Use i64 throughout to prevent potential overflow
        let mut freqs: HashMap<i64, i32> = HashMap::new();

        for &v in nums.iter() {
            *freqs.entry(v as i64).or_insert(0) += 1;
        }

        let mut memo: HashSet<Vec<i32>> = HashSet::new();
        let mut res = Vec::new();

        for x in 0..n.saturating_sub(2) {
            *freqs.get_mut(&(nums[x] as i64)).unwrap() -= 1;

            for y in (x + 1)..n.saturating_sub(1) {
                *freqs.get_mut(&(nums[y] as i64)).unwrap() -= 1;

                for z in (y + 1)..n {
                    *freqs.get_mut(&(nums[z] as i64)).unwrap() -= 1;

                    // Convert all numbers to i64 for the calculation
                    let m =
                        (target as i64) - (nums[x] as i64) - (nums[y] as i64) - (nums[z] as i64);

                    if let Some(&freq) = freqs.get(&m) {
                        if freq > 0 {
                            // Convert m back to i32 for the result array if within range
                            if m >= (i32::MIN as i64) && m <= (i32::MAX as i64) {
                                let mut quad = vec![nums[x], nums[y], nums[z], m as i32];
                                quad.sort();

                                if !memo.contains(&quad) {
                                    memo.insert(quad.clone());
                                    res.push(quad);
                                }
                            }
                        }
                    }

                    *freqs.get_mut(&(nums[z] as i64)).unwrap() += 1;
                }

                *freqs.get_mut(&(nums[y] as i64)).unwrap() += 1;
            }

            *freqs.get_mut(&(nums[x] as i64)).unwrap() += 1;
        }

        res.sort();
        res
    }
}
