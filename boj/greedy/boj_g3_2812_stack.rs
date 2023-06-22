use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    collections::VecDeque,
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

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached end of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);

    let (n, k) = (scanner.next::<usize>()?, scanner.next::<usize>()?);
    let size = n - k;

    let bytes = scanner.next::<String>()?.bytes().map(|b| b).collect::<Vec<u8>>();

    let mut cnt = 0;
    let mut stack = VecDeque::new();

    for digit in bytes {
        while cnt < k && !stack.is_empty() && *stack.back().unwrap() < digit {
            stack.pop_back();
            cnt += 1;
        }
        stack.push_back(digit);
    }

    for &digit in stack.iter().take(size) {
        write!(buf_writer, "{}", digit - 48)?;
    }
    Ok(())
}