// https://www.acmicpc.net/problem/1725
// O(n)

use std::io::{self, prelude::*, BufWriter};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let n = sc.read::<usize>();
    let table: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    let mut ans = 0;

    let mut stack = Vec::with_capacity(n);
    for i in 0..n {
        while !stack.is_empty() && table[*stack.last().unwrap()] > table[i] {
            let height = table[stack.pop().unwrap()];
            let width = if let Some(last) = stack.last() {
                i - last - 1
            } else { i };

            ans = usize::max(ans, width * height);
        }
        stack.push(i);
    }

    while !stack.is_empty() {
        let height = table[stack.pop().unwrap()];
        let width = if let Some(last) = stack.last() {
            n - last - 1
        } else { n };

        ans = usize::max(ans, width * height)
    }

    write!(output, "{}", ans)?;

    Ok(())
}