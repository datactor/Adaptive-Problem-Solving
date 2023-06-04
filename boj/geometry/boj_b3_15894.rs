// https://www.acmicpc.net/problem/15894

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    writeln!(output, "{}", n * 4)?;
    Ok(())
}