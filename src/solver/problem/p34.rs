pub struct P34;

impl P34 {
    pub fn solve_v1(nums: Vec<i32>, target: i32) -> Vec<i32>{
        let n = nums.len();
        let search = |x| {
            let mut left = 0;
            let mut right = n;
            while left < right {
                let mid = left + (right-left)/2;
                if nums[mid] < x {
                    left = mid + 1;
                }else {
                    right = mid;
                }
            }
            left
        };
        let left_i = search(target);
        let right_i = search(target + 1);
        if left_i == right_i {
            return vec![-1,-1];
        }
        vec![left_i as i32, (right_i-1) as i32]
        
    }
    pub fn solve_v2(nums: Vec<i32>, target: i32) -> Vec<i32>{
        fn binary_search(nums: &Vec<i32>, target: i32, is_first: bool) -> i32 {
            let mut start = 0;
            let mut end = nums.len();
            let mut i = -1;
            
            while start < end {
                let mid = start + (end - start) / 2;
                
                
                if nums[mid] == target {
                    i = mid as i32;
                    if is_first {
                        end = mid;
                    } else {
                        start = mid + 1;
                    }
                } else if nums[mid] < target {
                    start = mid + 1;
                } else {
                    end = mid;
                }
            }
            
            i
        }
        vec![binary_search(&nums, target, true), binary_search(&nums, target, false)]
    }
}