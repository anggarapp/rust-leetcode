
pub struct P38;

impl P38 {
    pub fn solve_v1(n: i32) -> String {
        use std::iter::once;
        (1..n).fold(vec![1], |curr, _|{
            let mut next = vec![];
            let mut slow = 0;
            for fast in 0..=curr.len() {
                if fast == curr.len() || curr[slow] != curr[fast] {
                    next.extend(once((fast - slow) as u8).chain(once(curr[slow])));
                    slow = fast;
                }
            }
            next
        })
        .into_iter()
        .map(|digit| (digit + b'0') as char)
        .collect()
    }
    pub fn solve_v2(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        
        let mut bytes = vec![b'1'];
        
        for _ in 1..n {
            bytes = Self::say(&bytes);
        }
        
        unsafe { String::from_utf8_unchecked(bytes) }
    }
    
    fn say(num: &[u8]) -> Vec<u8> {
        const ZERO: u8 = b'0';
        let mut left = 0;
        let mut cnt = vec![];

        for right in 1..=num.len() {
            if right == num.len() || num[left] != num[right] {
                cnt.push((right - left) as u8 + ZERO);
                cnt.push(num[left]);
                left = right;
            }
        }

        cnt
    }
}