// https://www.acmicpc.net/problem/9506

use std::{
    io::{self, prelude::*},
    error::Error,
    fmt::Write,
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

    loop {
        let n = sc.read::<i32>();
        if n == -1 { break };

        let mut buffer = String::new();

        let mut sum = 1;
        write!(buffer, "{} = 1", n)?;

        for i in 2..n {
            if n % i == 0 {
                sum += i;
                write!(buffer, " + {}", i)?;
            }
        }

        if sum == n {
            println!("{}", buffer);
        } else {
            println!("{} is NOT perfect.", n);
        }
    }
    Ok(())
}