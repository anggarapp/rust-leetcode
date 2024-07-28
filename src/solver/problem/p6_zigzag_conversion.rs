use crate::solver::Problem;

impl Problem {
    pub fn solve_zigzag_convertion_v1(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let _v_chars: Vec<char> = s.chars().collect();
        let _offside: i32 = num_rows + (num_rows - 3);
        let mut _counter: i32 = 0;
        let mut storage: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_rows {
            storage.push(Vec::new());
        }
        let mut backward: i32 = 0;
        for i in 0.._v_chars.len() {
            if _counter > _offside {
                _counter = 0;
            }
            if _counter >= num_rows {
                backward = backward + 1;
                storage[(num_rows - 1 - backward) as usize].push(_v_chars[i]);
            } else {
                backward = 0;
                storage[_counter as usize].push(_v_chars[i]);
            }
            _counter = _counter + 1;
        }
        let result: String = storage
            .into_iter()
            .flatten()
            .collect::<Vec<char>>()
            .into_iter()
            .collect();
        result
    }
}
