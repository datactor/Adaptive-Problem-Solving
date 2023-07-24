// https://www.acmicpc.net/problem/1049

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(buffer.as_bytes());
    let n = scanner.next::<usize>()?;
    let m = scanner.next::<usize>()?;

    let mut min_package = 6001;
    let mut min_each = 1001;

    for _ in 0..m {
        min_package = min_package.min(scanner.next::<usize>()?);
        min_each = min_each.min(scanner.next::<usize>()?);
    }
    min_package = min_package.min(min_each * 6);
    let mut ans = (n / 6) * min_package;
    let res = n % 6;
    if res != 0 {
        ans += min_package.min(res * min_each);
    }
    write!(buf_writer, "{}", ans)?;
    Ok(())
}
