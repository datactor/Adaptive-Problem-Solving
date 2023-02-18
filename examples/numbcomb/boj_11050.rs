use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().map(
        |s| s.parse::<usize>()).flatten();

    let n = v.next().unwrap();
    let k = v.next().unwrap();
    let mut ans = 1;
    for i in n-k+1..n+1 {
        ans *= i;
    }
    for i in 1..k+1 {
        ans /= i;
    }
    writeln!(output, "{}", ans).unwrap();
}