use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn next<T> (&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let mut cur_dir = 20i8;
    for _ in 0..10 {
        match scanner.next::<i8>()? {
            1 => cur_dir += 1,
            2 => cur_dir -= 2,
            _ => cur_dir -= 1,
        }
    }

    write!(buf_writer, "{}", match cur_dir % 4 {
        0 => "N",
        1 => "E",
        2 => "S",
        _ => "W",
    })?;

    Ok(())
}