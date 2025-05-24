pub struct P41;

impl P41 {
    pub fn solve_v1(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= (n as i32) && nums[i] != nums[(nums[i] - 1) as usize] {
                let j = (nums[i] - 1) as usize;
                nums.swap(i, j);
            }
        }
        
        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        (n + 1) as i32
    }
    pub fn solve_v2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort();
        let mut i = 0;
        while i < nums.len() {
            let correct_pos = nums[i] - 1;
            if nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[correct_pos as usize] {
                nums.swap(i, correct_pos as usize);
            } else {
                i += 1;
            }
        }

        for i in 0..nums.len() {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        return (nums.len()+1) as i32;
    }
}
