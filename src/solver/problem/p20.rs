pub struct P20;

impl P20 {
    pub fn solve_v1(s: String) -> bool {
        let mut stack = Vec::new();
        let par = vec!["()".to_string(), "[]".to_string(), "{}".to_string()];
        for c in s.chars() {
            if "({[".to_string().contains(c) {
                stack.push(c);
            } else if stack.is_empty() || !par.contains(&format!("{}{}", stack.pop().unwrap(), c)) {
                return false;
            }
        }
        stack.is_empty()
    }

    pub fn solve_v2(s: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert('(', ')');
        map.insert('{', '}');
        map.insert('[', ']');
        let mut stack = vec![];
        for c in s.chars() {
            if map.contains_key(&c) {
                stack.push(map[&c]);
            } else if stack.pop().unwrap_or(' ') != c {
                return false;
            }
        }
        stack.len() == 0
    }

    pub fn solve_v3(s: String) -> bool {
        use std::collections::HashMap;
        let s_len = s.len();
        if s_len == 0 || s_len % 2 != 0 {
            return false;
        }
        let bracket_map = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => stack.push(c),
                _ => {
                    let Some(last) = stack.last() else {
                        return false;
                    };
                    if c == bracket_map[last] {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }

        return stack.len() == 0;
    }
}
