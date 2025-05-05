pub struct P10;

impl P10 {
    pub fn solve(word: String, regx: String) -> bool {
        let s = word.as_bytes();
        let p = regx.as_bytes();
        let m = s.len();
        let n = p.len();

        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for j in 1..=n {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2]
                        || (dp[i - 1][j] && (p[j - 2] == s[i - 1] || p[j - 2] == b'.'));
                } else {
                    dp[i][j] = dp[i - 1][j - 1] && (p[j - 1] == s[i - 1] || p[j - 1] == b'.');
                }
            }
        }

        dbg!(&dp, &m, &n, &dp[m][n]);
        dp[m][n]
    }
}
