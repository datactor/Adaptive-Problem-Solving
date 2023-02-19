// https://www.acmicpc.net/problem/15651

use std::io::{self, prelude::*, BufWriter, StdoutLock};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();
    let n = v.next().unwrap();
    let m = v.next().unwrap();

    let mut arr = Vec::new();
    solve(n, m, &mut arr, &mut output);
}

fn solve(n: usize, m: usize, arr: &mut Vec<usize>, output: &mut BufWriter<StdoutLock>) {
    if arr.len() == m {
        for i in arr {
            write!(output, "{} ", i).unwrap();
        }
        write!(output, "\n").unwrap();
        return;
    }
    for i in 1..n + 1 {
        arr.push(i);
        solve(n, m, arr, output);
        arr.pop();
    }
}
