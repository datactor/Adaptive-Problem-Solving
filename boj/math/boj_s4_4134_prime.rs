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

    fn next<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
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

fn is_prime(x: usize) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    for i in 2..=((x as f64).sqrt() as usize) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(buffer.as_bytes());
    let t = scanner.next::<usize>()?;
    for _ in 0..t {
        let mut n = scanner.next::<usize>()?;
        while !is_prime(n) {
            n += 1;
        }
        writeln!(buf_writer, "{}", n)?;
    }

    Ok(())
}
