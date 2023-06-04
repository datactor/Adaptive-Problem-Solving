use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut factors = buffer
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    factors.sort();

    writeln!(output, "{}", factors[0] * factors[factors.len() - 1]).unwrap();
}
