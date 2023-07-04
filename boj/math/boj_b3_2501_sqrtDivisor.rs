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

fn get_divisor(n: usize, k: usize) -> usize {
    let mut divisors = Vec::new();
    let mut divisors_back = Vec::new();

    for i in 1..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors_back.push(n/i);
            }
        }
    }

    divisors.extend(divisors_back.into_iter().rev());
    if let Some(num) = divisors.get(k) {
        *num
    } else {
        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let k = scanner.next::<usize>()?;

    write!(buf_writer, "{}", get_divisor(n, k-1))?;
    Ok(())
}