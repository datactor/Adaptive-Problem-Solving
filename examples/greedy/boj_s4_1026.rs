// https://www.acmicpc.net/problem/1026

use std::{
    io::{self, BufRead, BufReader, Write, BufWriter},
};

fn read_to_vec(reader: &mut dyn BufRead, buffer: &mut String) -> io::Result<Vec<u32>> {
    buffer.clear();
    reader.read_line(buffer)?;

    let vec = buffer
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    Ok(vec)
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();

    // First line is skipped as it is not needed
    read_buf.read_line(&mut buffer)?;

    let mut a = read_to_vec(&mut read_buf, &mut buffer)?;
    a.sort();

    let mut b = read_to_vec(&mut read_buf, &mut buffer)?;
    b.sort_by(|a, b| b.cmp(a));

    let sum: u32 = a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .sum();

    writeln!(write_buf, "{}", sum)?;

    Ok(())
}