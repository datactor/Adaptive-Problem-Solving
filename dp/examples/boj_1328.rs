// https://www.acmicpc.net/problem/1328
// reference: https://lotuslee.tistory.com/118
// O(N * L * R)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

const MOD: usize = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, l, r) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());

    let mut dp = vec![vec![vec![0; r + 1]; l + 1]; n + 1];
    dp[1][1][1] = 1;

    // dp[n][l][r] = dp[n-1][l-1][r] + dp[n-1][l][r-1] + dp[n-1][l][r]*(n-2)
    for i in 2..=n {
        for j in 1..=l {
            for k in 1..=r {
                dp[i][j][k] =
                    (
                        (dp[i-1][j][k] * (i-2)) % MOD
                            + dp[i-1][j][k-1]
                            + dp[i-1][j-1][k]
                    ) % MOD;
            }
        }
    }

    writeln!(output, "{}", dp[n][l][r])?;

    Ok(())
}