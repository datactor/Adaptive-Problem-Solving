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
    let mut sc = Scanner::new(&buffer);
    let a = sc.next::<i32>()?;
    let b = sc.next::<i32>()?;
    let c = sc.next::<i32>()?;
    let d = sc.next::<i32>()?;
    let e = sc.next::<i32>()?;
    let f = sc.next::<i32>()?;

    let x = (c*e-b*f)/(a*e-b*d);
    let y = (c*d-a*f)/(b*d-a*e);

    write!(buf_writer, "{} {}", x, y)?;
    Ok(())
}