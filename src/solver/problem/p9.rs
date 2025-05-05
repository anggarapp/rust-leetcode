pub struct P9;
impl P9 {
    pub fn solve(x: i32) -> bool {
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
