use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt,
    ops::SubAssign,
    io::{self, Write, BufWriter},
    error::Error,
};

#[derive(PartialEq)]
struct BigInt(VecDeque<i8>);

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

impl BigInt {
    fn new() -> Self {
        Self(VecDeque::from([0]))
    }

    fn parse(input: &str) -> Self {
        Self(input.chars().rev().map(|c| c as i8 - '0' as i8).collect())
    }

    fn zero_justify(&mut self) {
        while self.0.len() > 1 && self.0.back() == Some(&0) {
            self.0.pop_back();
        }
    }

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.0.iter().rev() {
            let mut q = 0;

            dividend.0.push_front(num);
            dividend.zero_justify();

            while dividend >= other {
                dividend -= &other;
                q += 1;
            }

            quotient.0.push_front(q);
        }

        quotient.zero_justify();

        (quotient, dividend)
    }
}

impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, other: &Self) {
        let mut carry = 0;

        for i in 0..self.0.len() {
            let temp = carry + self.0[i] - other.0.get(i).unwrap_or(&0);

            if temp < 0 {
                carry = -1;
                self.0[i] = temp + 10;
            } else {
                carry = 0;
                self.0[i] = temp;
            }
        }

        self.zero_justify();
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() == other.0.len() {
            Some(self.0.iter().rev().cmp(other.0.iter().rev()))
        } else {
            Some(self.0.len().cmp(&other.0.len()))
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        write!(f, "")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_wrtier = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<String>()?;
    let m = scanner.next::<String>()?;
    let (a, b) = BigInt::parse(&n).divmod(BigInt::parse(&m));

    write!(buf_wrtier, "{}\n{}", a, b)?;
    Ok(())
}
