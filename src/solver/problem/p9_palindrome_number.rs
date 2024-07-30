use crate::solver::Problem;

impl Problem {
    pub fn solve_palindrom_number_v1(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let mut y: i32 = 0;
        let mut z = x.clone();
        while z > 0 {
            y = y * 10 + z % 10;
            z = z / 10;
        }
        return y == x;
    }
}
