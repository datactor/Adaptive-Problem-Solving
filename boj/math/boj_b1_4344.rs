// https://www.acmicpc.net/problem/4344

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    error::Error,
};

macro_rules! reads {
    ($reader:expr, $input:expr) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(io::stdin().lock());
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();
    let n = reads!(buf_reader, buf_to_string).parse::<usize>()?;

    for _ in 0..n {
        let v: Vec<i32> = reads!(buf_reader, buf_to_string)
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let avg = (v[1..].iter().sum::<i32>()) as f32 / v[0] as f32;
        let cnt = (v[1..]).iter().filter(|x| **x as f32 > avg).count();
        writeln!(buf_writer, "{:.3}%", (cnt as f32/v[0] as f32) * 100.0)?;
    }
    Ok(())
}