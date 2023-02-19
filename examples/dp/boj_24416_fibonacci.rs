// https://www.acmicpc.net/problem/24416

use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    // for문으로 풀기
    let mut arr = [1; 40];
    for i in 2..n {
        arr[i] = arr[i - 1] + arr[i - 2];
    }
    writeln!(output, "{} {}", arr[n - 1], n - 2).unwrap();

    // writeln!(output, "{} {}", fib(n), n-2).unwrap();
}

// 문제 그대로 재귀로 풀기
fn fib(n: usize) -> usize {
    if n == 1 || n == 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
