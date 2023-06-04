use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut iter = input.split_ascii_whitespace();
    let mut next = || iter.next().unwrap().parse::<usize>().unwrap();
    let (n, m) = (next(), next());
    let mut basket = vec![0; n+1];

    for _ in 0..m {
        let (i, j, k) = (next(), next(), next());
        for idx in i..=j {
            basket[idx] = k
        }
    }

    for i in 1..=n {
        write!(output, "{} ", basket[i])?;
    }

    Ok(())
}