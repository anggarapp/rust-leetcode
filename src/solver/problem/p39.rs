pub struct P39;

impl P39 {
    pub fn solve_v1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut t: Vec<i32> = vec![];
        let mut results: Vec<Vec<i32>> = vec![];
        fn dfs(candidates: &mut Vec<i32>, i: i32, s: i32, t: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
            if s == 0{
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
    
}