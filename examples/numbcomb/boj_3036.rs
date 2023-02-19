use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();

    let n = v.next().unwrap();
    let first_ring = v.next().unwrap();
    for _ in 0..n - 1 {
        let num = v.next().unwrap();
        let gcd = find_gcd(first_ring, num);
        writeln!(output, "{}/{}", first_ring / gcd, num / gcd).unwrap();
    }
}

fn find_gcd(mut num: usize, mut div: usize) -> usize {
    let mut res = num % div;
    while res != 0 {
        num = div;
        div = res;
        res = num % div;
    }
    div
}
