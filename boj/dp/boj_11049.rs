// https://www.acmicpc.net/problem/11049

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let n = sc.read::<usize>();

    let matrixes: Vec<(usize, usize)> = (0..n).map(|_| (sc.read::<usize>(), sc.read::<usize>())).collect();

    // initialize the dp table with zeros
    let mut dp = vec![vec![0; n]; n];

    // loop through all possible submatrices (in order of size)
    for s in 0..n-1 {
        for i in 0..n-1-s {
            let j = i+s+1;
            dp[i][j] = usize::MAX; // initialize with a maximum value
            for k in i..j {
                // update the value of dp[i][j] based on the optimal split point k
                dp[i][j] = usize::min(dp[i][j], dp[i][k] + dp[k+1][j] + matrixes[i].0*matrixes[k].1*matrixes[j].1);
            }
        }
    }

    writeln!(output, "{}", dp[0].last().unwrap())?;

    Ok(())
}