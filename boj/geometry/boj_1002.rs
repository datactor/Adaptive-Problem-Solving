// https://www.acmicpc.net/problem/1002

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let n = lines.next().ok_or("No lines")?.parse::<usize>()?;

    for _ in 0..n {
        let mut v = lines
            .next()
            .ok_or("Reached out of lines")?
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap());
        let (x1, y1, r1) = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
        let (x2, y2, r2) = (v.next().unwrap(), v.next().unwrap(), v.next().unwrap());

        let dst = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f32).powf(0.5);
        let ans = if dst == 0.0 && r1 == r2 {
            -1
        } else if dst == (r1 + r2) as f32 || (r1 - r2).abs() as f32 == dst {
            1
        } else if dst < (r1 + r2) as f32 && dst > (r1 - r2).abs() as f32 {
            2
        } else {
            0
        };
        writeln!(output, "{}", ans)?;
    }
    Ok(())
}