// https://www.acmicpc.net/problem/1007

use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = lines.next().unwrap().parse::<usize>().unwrap();
        let p: Vec<(i64, i64)> = (0..n)
            .map(|_| {
                let mut v = lines
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i64>())
                    .flatten();
                (v.next().unwrap(), v.next().unwrap())
            })
            .collect();

        let min = ((v_match(&p, (0, 0), n / 2)) as f64).sqrt();

        writeln!(output, "{:.6}", min)?;
    }
    Ok(())
}

fn v_match(p: &[(i64, i64)], (mut tx, mut ty): (i64, i64), left: usize) -> i64 {
    match left {
        0 => {
            for &(x, y) in p {
                tx += x;
                ty += y;
            }
            return tx.pow(2) + ty.pow(2);
        }
        left if left == p.len() => {
            for &(x, y) in p {
                tx -= x;
                ty -= y;
            }
            return tx.pow(2) + ty.pow(2);
        }
        _ => {
            return v_match(&p[1..], (tx + p[0].0, ty + p[0].1), left).min(v_match(
                &p[1..],
                (tx - p[0].0, ty - p[0].1),
                left - 1,
            ))
        }
    }
}
