pub struct P27;

impl P27 {
    pub fn solve_v1(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
    pub fn solve_v2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j = 0;
        for i in 0..nums.len() {
            nums[i - j] = nums[i];
            if nums[i] == val {
                j += 1;
            }
        }
        (nums.len() - j).try_into().unwrap()
    }
}
