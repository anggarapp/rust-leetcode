pub struct P39;

impl P39 {
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
            if s < candidates[i as usize] {
                return;
            }
            for j in (i as usize)..candidates.len() {
                t.push(candidates[j]);
                dfs(candidates, j as i32, s - candidates[j], t, results);
                t.pop();
            }
        }
        dfs(&mut candidates, 0, target, &mut t, &mut results);

        // results.sort_by(|a, b| b.cmp(a));
        results
    }

    fn dfs(i: usize, s: i32, candidates: &Vec<i32>, t: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if s == 0 {
            ans.push(t.clone());
            return;
        }
        if s < candidates[i] {
            return;
        }
        for j in i..candidates.len() {
            t.push(candidates[j]);
            Self::dfs(j, s - candidates[j], candidates, t, ans);
            t.pop();
        }
    }

    pub fn solve_v2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = Vec::new();
        Self::dfs(0, target, &candidates, &mut vec![], &mut ans);
        ans
    }

    fn generate(
        cur: &mut Vec<i32>,
        start: usize,
        sum: i32,
        target: i32,
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if sum > target {
            return;
        }

        if sum == target {
            ans.push(cur.clone());
            return;
        }

        for i in start..candidates.len() {
            cur.push(candidates[i]);

            Self::generate(cur, i, sum + candidates[i], target, candidates, ans);

            cur.pop();
        }
    }

    pub fn solve_v3(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        let mut cur = vec![];

        Self::generate(&mut cur, 0, 0, target, &candidates, &mut results);

        results
    }


}
