use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.split_ascii_whitespace(),
        }
    }

    fn next<T> (&mut self) -> Result<T, Box<dyn Error>> 
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("EOF")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;

    let mut scanner = Scanner::new(&buf);
    let n = scanner.next::<i32>()?;
    let l = scanner.next::<i32>()?;

    for l in l..=100 {
        let temp_diff = n - l * (l + 1) / 2;
        if temp_diff % l == 0 {
            let x = temp_diff / l + 1;
            if x >= 0 {
                for len in 0..l {
                    write!(buf_writer, "{} ", len + x)?;
                }
                return Ok(());
            }
        }
    }
    
    write!(buf_writer, "-1")?;
    Ok(())
}