use std::io::prelude::*;
use std::io::{self, BufWriter};

struct Scanner<'a> {
    inner: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(buf: &'a str) -> Scanner<'a> {
        Scanner {
            inner: buf.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.inner.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut output = BufWriter::new(io::stdout().lock());
    let mut scanner = Scanner::new(&buffer);
    let mut n = scanner.read::<usize>();

    writeln!(output, "{}", i32::pow(2, n as u32)-1);
    solve(n, &mut output, 1, 3);
    output.flush().unwrap();
}

fn solve<W: Write>(n: usize, output: &mut BufWriter<W>, l: usize, r: usize) {
    if n == 1 {
        writeln!(output, "{} {}", l, r);
        return
    }
    solve(n-1, output, l, 6-l-r);
    writeln!(output, "{} {}", l, r);
    solve(n-1, output, 6-l-r, r);
}