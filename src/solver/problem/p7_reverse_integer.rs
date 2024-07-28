use crate::solver::Problem;

impl Problem {
    pub fn solve_reverse_integer_v1(num: i32) -> i32 {
        let mut revs: i64 = 0;
        let mut int: i64 = if num < 0 { num as i64 * -1 } else { num as i64 };
        while int > 0 {
            revs *= 10;
            revs += int % 10;
            if revs > i32::MAX as i64 || revs < i32::MIN as i64 {
                return 0;
            }
            int /= 10;
        }
        if num < 0 {
            (revs as i32) * -1
        } else {
            revs as i32
        }
    }
}
