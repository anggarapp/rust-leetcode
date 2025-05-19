pub struct P32;
impl P32{
    pub fn solve_v1(s: String) -> i32 {
        let mut result = 0;
        let mut func = vec![0; s.len()+1];
        for i in 2..=s.len() {
            if s.chars().nth(i-1).unwrap() == ')' {
                if s.chars().nth(i-2).unwrap() == '(' {
                    func[i] = func[i-2] +2;
                } else if (i as i32) - func[i-1] -1 > 0 && s.chars().nth(i - (func[i-1] as usize) -2).unwrap() == '(' {
                    func[i] = func[i-1] +2 + func[i- (func[i-1] as usize) - 2];
                }
                result = result.max(func[i]);
            }
        }
        result
    }
    pub fn solve_v2(s: String) -> i32 {
        let chars = s.as_bytes();
        
        let mut stack = vec![];
        let mut max = 0;
        let mut current_start = 0;
        for i in 0..chars.len() {
            let c = chars[i];
            if c == ')' as u8 {
                if let Some(x) = stack.pop() {
                    let len = i - x + 1;
                    max = if max < len {len} else {max};
                    current_start = x;
                }
                else {
                    let len = i - current_start;
                    max = if max < len {len} else {max};
                    current_start = i+1;
                }
            }
            if c == '(' as u8 {
                stack.push(current_start);
                current_start = i+1;
            }
        }
        
        return max as i32;
    }

    pub fn solve_v3(s: String) -> i32 {
        let mut max = 0;
        let ch = s.as_bytes();
        for i in 0..ch.len() {
            if ch[i] == b'(' {
                let mut n = 1;
                for j in i+1..ch.len() {
                    n += match ch[j] {
                        b'(' => 1,
                        b')' => -1,
                        _ => 0
                    };
                    if n < 0 {
                        break;
                    }
                    if n == 0 && j-i+1 > max{
                        max = j-i+1;
                    }
                }
            }
        }

        max.try_into().unwrap_or(-1)
    }
}