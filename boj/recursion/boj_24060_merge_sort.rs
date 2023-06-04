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
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut scanner = Scanner::new(&buffer);
    let mut output = BufWriter::new(io::stdout().lock());
    solve(&mut scanner, &mut output);
    output.flush().unwrap();
}

fn solve<W: Write>(scanner: &mut Scanner, output: &mut BufWriter<W>) {
    let n = scanner.read::<usize>();
    let mut k = scanner.read::<usize>() - 1;
    let mut array: Vec<i32> = (0..n).map(|_| scanner.read()).collect();
    let mut v = vec![];

    rec(0, n, &mut v);

    let mut ans = -1;
    for &(l, r) in v.iter() {
        let len = r - l;
        if k < len {
            array[l..r].sort();
            ans = array[l + k];
            break;
        } else {
            k -= len;
        }
    }
    writeln!(output, "{}", ans).unwrap();
}

fn rec(l: usize, r: usize, v: &mut Vec<(usize, usize)>) {
    if r - l > 1 {
        let mid = (l + r + 1) / 2;
        rec(l, mid, v);
        rec(mid, r, v);
        v.push((l, r));
    }
}
