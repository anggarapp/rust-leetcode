pub struct P17;

impl P17 {
    pub fn solve_v1(digits: String) -> Vec<String> {
        let mut answer: Vec<String> = Vec::new();
        if digits.is_empty() {
            return answer;
        }

        answer.push("".to_string());

        let digits_map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        for i in digits.chars() {
            let s = &digits_map[((i as u8) - b'2') as usize];
            let mut t: Vec<String> = Vec::new();
            for a in &answer {
                for b in s.chars() {
                    t.push(format!("{}{}", a, b));
                }
            }
            answer = t;
        }
        answer
    }

    pub fn solve_v2(digits: String) -> Vec<String> {
        let digits_map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut answer = Vec::new();
        let mut t = String::new();
        if digits.is_empty() {
            return answer;
        }
        P17::dfs(&digits, &digits_map, &mut t, &mut answer, 0);
        answer
    }

    fn dfs(digits: &String, d: &[&str; 8], t: &mut String, ans: &mut Vec<String>, i: usize) {
        if i >= digits.len() {
            ans.push(t.clone());
            return;
        }

        let s = d[((digits.chars().nth(i).unwrap() as u8) - b'2') as usize];
        for c in s.chars() {
            t.push(c);
            P17::dfs(digits, d, t, ans, i + 1);
            t.pop();
        }
        return;
    }

    pub fn solve_v3(digits: String) -> Vec<String> {
        if digits.len() <= 0 {
            return Vec::new();
        }

        let mut n = 1;
        let mut pad: Vec<&str> = Vec::with_capacity(4);
        for c in digits.chars() {
            match c {
                '2' => {
                    pad.push("abc");
                    n *= 3;
                }
                '3' => {
                    pad.push("def");
                    n *= 3;
                }
                '4' => {
                    pad.push("ghi");
                    n *= 3;
                }
                '5' => {
                    pad.push("jkl");
                    n *= 3;
                }
                '6' => {
                    pad.push("mno");
                    n *= 3;
                }
                '7' => {
                    pad.push("pqrs");
                    n *= 3;
                }
                '8' => {
                    pad.push("tuv");
                    n *= 3;
                }
                '9' => {
                    pad.push("wxyz");
                    n *= 3;
                }
                _ => {
                    println!("unknown digit!");
                }
            }
        }
        let mut results: Vec<String> = vec!["".to_string(); 1];
        let mut prev: Vec<String>;
        for i in 0..pad.len() {
            prev = results;
            n *= pad[i].len();
            results = Vec::with_capacity(n);
            for w in prev {
                for c in pad[i].chars() {
                    let mut nw: String = w.clone();
                    nw.push(c);
                    results.push(nw);
                }
            }
        }
        results
    }
}
