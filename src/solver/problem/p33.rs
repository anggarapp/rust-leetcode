pub struct P33;

impl P33 {
    pub fn solve_v1(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n-1;
        
        while left< right {
            let mid = (left +right) >> 1;
            if nums[0] <= nums[mid]{
                if nums[0] <= target && target <= nums[mid]{
                    right = mid;
                } else {
                    left = mid +1;
                }
            }else {
                if nums[mid] < target && target <= nums[n-1]{
                    left = mid +1;
                }else {
                    right = mid;
                }
            }
        }
        if nums[left] != target{
            return -1;
        }
        left as i32
    }
    
    pub fn solve_v2(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let guess = nums[mid];

            if target == guess {
                return mid as i32;
            }
            // check left sorted portion
            if nums[left] <= guess {
                if target > guess || target < nums[left] {
                    // Search in right portion
                    left = mid + 1;
                } else {
                    // Search in left portion
                    right = mid - 1;
                }
            } else {
                if target < guess || target > nums[right] {
                    // Search in left portion
                    right = mid - 1;
                } else {
                    // Search in right portion
                    left = mid + 1;
                }
            }
        }
        -1
    }
}