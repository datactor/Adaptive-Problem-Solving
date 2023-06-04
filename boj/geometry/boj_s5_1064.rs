// https://www.acmicpc.net/problem/1064

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let ax = numbers[0];
    let ay = numbers[1];
    let bx = numbers[2];
    let by = numbers[3];
    let cx = numbers[4];
    let cy = numbers[5];

    if (ax - bx) * (ay - cy) == (ay - by) * (ax - cx) {
        writeln!(output, "{}", -1.0)?;
        return Ok(());
    }

    let ab_len = (((ax - bx).pow(2) + (ay - by).pow(2)) as f64).sqrt();
    let ac_len = (((ax - cx).pow(2) + (ay - cy).pow(2)) as f64).sqrt();
    let bc_len = (((bx - cx).pow(2) + (by - cy).pow(2)) as f64).sqrt();

    let lengths = [ab_len + ac_len, ab_len + bc_len, ac_len + bc_len];
    let result = lengths.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap()
        - lengths.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
    writeln!(output, "{:.9}", 2.0 * result)?;

    Ok(())
}