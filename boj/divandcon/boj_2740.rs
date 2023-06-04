use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let mut n_m = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .flatten();
    let (n, m) = (n_m.next().unwrap(), n_m.next().unwrap());

    let mut a: Vec<Vec<i32>> = (0..n)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut k = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut b: Vec<Vec<i32>> = (0..m)
        .map(|_| {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    for r in a {
        for i in 0..k {
            let sum: i32 = r.iter().enumerate().map(|(j, num)| num * b[j][i]).sum();
            write!(output, "{} ", sum)?;
        }
        writeln!(output)?;
    }

    Ok(())
}
