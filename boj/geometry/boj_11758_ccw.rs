// https://www.acmicpc.net/problem/11758
// ccw? https://johoonday.tistory.com/102

use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
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

    fn ccw(&mut self) -> i32 {
        let p1 = (self.read::<i32>(), self.read::<i32>());
        let p2 = (self.read::<i32>(), self.read::<i32>());
        let p3 = (self.read::<i32>(), self.read::<i32>());

        return (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let result = sc.ccw();

    writeln!(
        output,
        "{}",
        match result {
            0 => 0,
            result if 0 > result => -1,
            _ => 1,
        }
    )?;

    Ok(())
}
