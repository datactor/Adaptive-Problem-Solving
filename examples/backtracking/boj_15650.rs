// https://www.acmicpc.net/problem/15650
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

fn solve(n: usize, m: usize, v: &mut Vec<usize>, output: &mut BufWriter<StdoutLock>) {
    if v.len() == m {
        for i in v {
            write!(output, "{} ", i).unwrap();
        }
        write!(output, "\n").unwrap();
        return;
    }

    for i in 1..n + 1 {
        if v.len() != 0 && v[v.len() - 1] >= i {
            continue;
        }
        // iter 최대 3(lg8)이라 상관은 없음
        // match v.iter().max() {
        //     Some(s) if s >= &i => continue,
        //     _ => {},
        // }

        v.push(i);
        solve(n, m, v, output);
        v.pop();
    }
}
