use std::io::{self, prelude::*, BufWriter};

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

    let two = count(n, 2) - count(m, 2) - count(n - m, 2);
    let five = count(n, 5) - count(m, 5) - count(n - m, 5);

    writeln!(output, "{}", two.min(five)).unwrap();
}

fn count(mut n: usize, k: usize) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= k;
        count += n;
    }
    count
}
