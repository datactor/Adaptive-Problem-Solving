// https://www.acmicpc.net/problem/7568

use std::io::{self, prelude::*, BufWriter};

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
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut output = BufWriter::new(io::stdout().lock());
    let mut scanner = Scanner::new(&buffer);
    solve(&mut scanner, &mut output);
    buffer.clear();
    output.flush().unwrap();
}

fn solve<W: Write>(scanner: &mut Scanner, output: &mut BufWriter<W>) {
    let n = scanner.read::<usize>();
    let mut array = (0..n)
        .map(|_| (scanner.read(), scanner.read()))
        .collect::<Vec<(usize, usize)>>();
    for i in &array {
        let cnt = (&array)
            .into_iter()
            .filter(|s| s.0 > i.0 && s.1 > i.1)
            .count();
        write!(output, "{} ", cnt + 1);
    }
}
