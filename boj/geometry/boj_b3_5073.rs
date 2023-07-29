// https://www.acmicpc.net/problem/5073

use std::{
    error::Error,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(io::stdin().lock());
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::from("0");
    while !buffer.is_empty() {
        buffer.clear();
        buf_reader.read_line(&mut buffer)?;
        let mut edges = buffer
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        edges.sort_unstable();
        if edges[0] + edges[1] <= edges[2] {
            if edges[2] == 0 {
                break;
            }
            writeln!(buf_writer, "Invalid")?;
            continue;
        }
        edges.dedup();
        let state = match edges.len() {
            1 => "Equilateral",
            2 => "Isosceles",
            _ => "Scalene",
        };
        writeln!(buf_writer, "{}", state)?;
    }
    Ok(())
}
