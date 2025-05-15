pub struct P29;

impl P29 {
    pub fn solve_v1(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        if dividend == std::i32::MIN && divisor == -1 {
            return std::i32::MAX;
        }
        let sign = if (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
            true
        } else {
            false
        };
        let mut dividend = if dividend > 0 {
            -dividend
        }else {
            dividend
        };
        let divisor = if divisor > 0 {
            -divisor
        }else {
            divisor
        };
        let mut result = 0;
        while dividend <= divisor {
            let mut x = divisor;
            let mut counter = 1;
            while x >= (std::i32::MIN >> 1) && dividend <= (x <<1) {
                x <<=1;
                counter<<= 1;
            }
            dividend -=x;
            result += counter
        }
        if sign {
            return result;
        } else {
            return -result;
        }
    }
}
