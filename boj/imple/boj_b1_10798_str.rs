// https://www.acmicpc.net/problem/10798

use std::{
    error::Error,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(io::stdin().lock());
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::from("0");
    let mut vec = Vec::new();

    let mut n = 0;

    while !buffer.is_empty() {
        buffer.clear();
        buf_reader.read_line(&mut buffer)?;

        n = n.max(buffer.len());
        vec.push(buffer.as_bytes().to_vec());
    }

    for i in 0..n {
        for j in 0..vec.len() {
            if let Some(c) = vec[j].get(i) {
                if c == &b'\n' {
                    continue;
                }
                buf_writer.write(&[*c])?;
            }
        }
    }
    Ok(())
}
