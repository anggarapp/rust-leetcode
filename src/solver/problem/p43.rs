pub struct P43;

impl P43 {
    pub fn solve_v1(num_one: String, num_two: String) -> String {
        if num_one == "0" || num_two == "0" {
            return "0".to_string();
        }
        let n_one = num_one.chars().rev().collect::<String>();
        let n_two = num_two.chars().rev().collect::<String>();
        let mut result = vec![0; n_one.len() + n_two.len()];
        for (i_one, c_one) in n_one.chars().enumerate() {
            for (i_two, c_two) in n_two.chars().enumerate() {
                let digit = c_one.to_digit(10).unwrap_or(0) * c_two.to_digit(10).unwrap_or(0);
                result[i_one + i_two] += digit;
                result[i_one + i_two + 1] += result[i_one + i_two] / 10;
                result[i_one + i_two] %= 10;
            }
        }

        while let Some(&0) = result.last() {
            result.pop();
        }

        result
            .into_iter()
            .rev()
            .map(|digit| char::from_digit(digit as u32, 10).unwrap())
            .collect()
    }
    pub fn solve_v2(num_one: String, num_two: String) -> String {
        match (num_one.as_str(), num_two.as_str()) {
            (_, _) if num_one == "0" || num_two == "0" => return "0".to_string(),
            ("1", _) => return num_two,
            (_, "1") => return num_one,
            _ => {}
        }

        let n = num_one.len();
        let m = num_two.len();
        let mut res = vec![0u16; n + m];

        for (i, da) in num_one.as_bytes().iter().rev().enumerate() {
            let da = (da - b'0') as u16;
            for (j, db) in num_two.as_bytes().iter().rev().enumerate() {
                let db = (db - b'0') as u16;
                res[i + j] += da * db;
            }
        }

        let mut carry = 0u16;
        for digit in &mut res {
            let tmp = *digit + carry;
            *digit = tmp % 10;
            carry = tmp / 10; // max 9, weil tmp < 10 × 10 + 9
        }

        match res.iter().rposition(|&c| c != 0) {
            Some(pos) => res.truncate(pos + 1),
            None => res.truncate(1),
        }

        let out = res
            .iter()
            .rev()
            .map(|&d| char::from(d as u8 + b'0'))
            .collect();
        out
    }
}
