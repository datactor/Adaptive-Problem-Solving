// https://www.acmicpc.net/problem/2293

use std::{
    io::{self, prelude::*},
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
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (n, k) = (sc.read::<usize>(), sc.read::<usize>());

    let mut dp = vec![0; k+1];
    dp[0] = 1;

    (0..n).for_each(|_| {
            let coin = sc.read::<usize>();
            for value in 1..k+1 {
                if value >= coin {
                    dp[value] += dp[value - coin];
                }
            }
        }
    );

    println!("{}", dp[k]);

    Ok(())
}