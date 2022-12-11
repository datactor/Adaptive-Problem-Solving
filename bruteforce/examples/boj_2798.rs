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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut output = BufWriter::new(io::stdout().lock());
    let mut scanner = Scanner::new(&input);
    solve(&mut scanner, &mut output);
    output.flush().unwrap();
}

fn solve<W: Write>(scanner: &mut Scanner, output: &mut BufWriter<W>) {
    let n = scanner.read::<usize>();
    let m = scanner.read::<usize>();

    let mut max = 0;
    let mut array = (0..n).map(
        |_| scanner.read()).collect::<Vec<usize>>();

    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                let tmp = array[i] + array[j] + array[k];
                if tmp <= m {
                    if tmp == m {
                        writeln!(output, "{}", tmp).unwrap();
                        return
                    } else if max < tmp {
                        max = tmp;
                    }
                }
            }
        }
    }
    writeln!(output, "{}", max).unwrap();
}