// https://www.acmicpc.net/problem/2559
// O(N)

use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();

    let mut a = lines.next().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();
    let n = a.next().unwrap();
    let k = a.next().unwrap();

    let v = lines.next().take().unwrap().split_ascii_whitespace().map(
        |s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut sum = v[0..k].iter().sum::<i32>();
    let mut max = sum;
    for i in k..n {
        sum += v[i] - v[i-k];
        max = max.max(sum);
    }
    writeln!(output, "{}", max).unwrap();
}