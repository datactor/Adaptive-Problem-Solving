// https://www.acmicpc.net/problem/9461

use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();

    let mut arr: [i64; 101] = [1; 101];

    (arr[4], arr[5]) = (2, 2);

    for i in 6..101 {
        arr[i] = arr[i-5] + arr[i-1];
    }

    for _ in 0..n {
        let num = v.next().unwrap();
        writeln!(output, "{}", arr[num]).unwrap();
    }
}