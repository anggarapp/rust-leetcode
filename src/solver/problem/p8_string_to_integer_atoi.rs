use std::i32;

use crate::solver::Problem;

impl Problem {
    pub fn solve_string_to_integer_atoi_v1(s: String) -> i32 {
        let _trimed_string = s.trim();
        let chars = _trimed_string.chars().nth(0);
        let mut operator = 1;
        match chars {
            Some('+') => operator = 1,
            Some('-') => operator = -1,
            _ => (),
        }
        let mut clean_string = _trimed_string.to_string();
        if _trimed_string.len() > 0 && (chars == Some('+') || chars == Some('-')) {
            clean_string.remove(0);
        }
        let dirty_number: Vec<char> = clean_string.chars().into_iter().collect();
        dbg!(&dirty_number);
        let mut clear_number: Vec<char> = Vec::new();
        for character in dirty_number {
            match character {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    clear_number.push(character);
                }
                _ => break,
            }
        }
        if clear_number.len() < 1 {
            return 0;
        }
        let num_str: String = clear_number.into_iter().collect();
        if operator > 0 {
            num_str.parse::<i32>().unwrap_or(i32::MAX)
        } else {
            match num_str.parse::<i32>() {
                Ok(num) => return num * operator,
                Err(_) => return i32::MIN,
            }
        }
    }

    pub fn solve_string_to_integer_atoi_vx(s: String) -> i32 {
        let mut result: i64 = 0;
        let trimmed = s.trim();

        //check for sign
        let is_negative = trimmed.starts_with('-');
        let start_at = if trimmed.starts_with('+') || is_negative {
            1
        } else {
            0
        };

        for char in trimmed[start_at..].chars() {
            //println!("{}", char);

            if char.is_ascii_digit() {
                let num = char.to_digit(10).unwrap() as i64;
                result *= 10;
                result += num;

                if result > i32::max_value().into() && !is_negative {
                    return i32::max_value();
                }
                if result > i32::max_value().into() && is_negative {
                    return i32::min_value();
                }
            } else {
                break;
            }
        }

        let signed_result = if is_negative { -result } else { result };

        signed_result as i32
    }
}
