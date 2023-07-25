// https://www.acmicpc.net/problem/2485

use std::{
    error::Error,
    io::{self, BufWriter, Read, Write},
};

struct Scanner<'a> {
    reader: Box<dyn Read + 'a>,
    buf: Vec<u8>,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new<T: Read + 'a>(reader: T) -> Self {
        Self {
            reader: Box::new(reader),
            buf: Vec::new(),
            pos: 0,
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn Error>> {
        loop {
            if let Some(i) = self.buf[self.pos..]
                .iter()
                .position(|&c| c == b' ' || c == b'\n')
            {
                let res = std::str::from_utf8(&self.buf[self.pos..self.pos + i])
                    .unwrap()
                    .parse::<T>()
                    .ok()
                    .expect("parse fail");
                self.pos += i + 1;
                return Ok(res);
            }
            self.buf.clear();
            self.reader.read_to_end(&mut self.buf).expect("read fail");
            self.pos = 0;
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(buffer.as_bytes());
    let n = scanner.next::<usize>()?;
    let mut interval = Vec::with_capacity(n - 1);
    let pivot = scanner.next::<i32>()?;
    let mut tmp = pivot;
    for _ in 1..n {
        let colonnade = scanner.next::<i32>()?;
        interval.push(colonnade - tmp);
        tmp = colonnade;
    }

    let mut max = gcd(interval[0], interval[1]);
    for i in 2..interval.len() {
        max = gcd(max, interval[i]);
    }

    write!(buf_writer, "{}", (tmp - pivot) / max - n as i32 + 1)?;
    Ok(())
}
