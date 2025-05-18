pub struct P31;

impl P31 {
    pub fn solve_v1(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut i = (len - 1) as i32;
        let mut prev = -1;
        while i >= 0 {
            if nums[i as usize] < prev {
                break;
            }
            prev = nums[i as usize];
            i -= 1;
        }
        let mut j = len - 1;
        if i >= 0 {
            while j > (i as usize) {
                if nums[j] > nums[i as usize] {
                    nums.swap(i as usize, j);
                    break;
                }
                j -= 1;
            }
        }
        let slice = &mut nums[((i + 1) as usize)..len];
        slice.reverse();
    }

    pub fn solve_v2(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = nums.len() - 2;
        while nums[i] >= nums[i + 1] {
            if i == 0 {
                break;
            }

            i -= 1;
        }

        if i == 0 && nums[0] >= nums[1] {
            nums.reverse();
            return;
        }

        let mut j = nums.len() - 1;
        while nums[j] <= nums[i] {
            j -= 1;
        }

        nums.swap(i, j);
        Self::reverse_from(nums, i + 1);
    }

    fn reverse_from(nums: &mut Vec<i32>, i: usize) {
        let mut left = i;
        let mut right = nums.len() - 1;

        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    pub fn solve_v3(nums: &mut Vec<i32>) {
        let n = nums.len();
        if let Some(i) = (1..n).rev().find(|&i| nums[i-1] < nums[i]){
            let j = (i..n).rev().find(|&j| nums[i-1] <  nums[j]).unwrap();
            nums.swap(i-1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
        }
    }
}
