// https://www.acmicpc.net/problem/2903

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
            self.reader.read_to_end(&mut self.buf)?;
            self.pos = 0;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(buffer.as_bytes());
    let n = scanner.next::<usize>()?;
    let mut vec = vec![0; 16];
    vec[0] = 4;
    vec[1] = 9;
    let mut x = 3;
    for i in 2..=n {
        x = x * 2 - 1;
        vec[i] = (vec[i - 1] - x) * 4 + x * 2 - 1;
    }
    write!(buf_writer, "{}", vec[n])?;
    Ok(())
}
