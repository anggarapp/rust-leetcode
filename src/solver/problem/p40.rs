pub struct P40;

impl P40 {
    pub fn solve_v1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut t: Vec<i32> = vec![];
        let mut results: Vec<Vec<i32>> = vec![];
        fn dfs(
            candidates: &mut Vec<i32>,
            i: i32,
            s: i32,
            t: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            if s == 0 {
                results.push(t.clone());
                return;
            }
            if i >= candidates.len() as i32 || s < candidates[i as usize] {
                return;
            }
            for j in (i as usize)..candidates.len() {
                if j > (i as usize) && candidates[j] == candidates[j - 1] {
                    continue;
                }
                t.push(candidates[j]);
                dfs(candidates, (j + 1) as i32, s - candidates[j], t, results);
                t.pop();
            }
        }
        dfs(&mut candidates, 0, target, &mut t, &mut results);
        results
    }

    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        store: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
    ) {
        if target == 0 {
            result.push(store.clone());
            return;
        }
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            if target - candidates[i] < 0 {
                return;
            }
            store.push(candidates[i].clone());
            Self::backtrack(result, store, candidates, target - candidates[i], i + 1);
            store.pop();
        }
    }

    pub fn solve_v2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut store = Vec::new();
        let mut candidates = candidates;
        candidates.sort();
        Self::backtrack(&mut result, &mut store, &candidates, target, 0);
        return result;
    }
}
