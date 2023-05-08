// https://www.acmicpc.net/problem/11664

use std::{
    io::{self, prelude::*, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    num::{ParseIntError as IE, ParseFloatError as FE},
    fmt,
};

trait Parser {
    fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
}

impl<'a> Parser for SplitAsciiWhitespace<'a> {
    fn read<T, E>(&mut self) -> T
        where
            T: FromStr<Err = E>,
            E: fmt::Debug,
    {
        self.next().expect("EOF").parse().expect("Parse Error")
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_ascii_whitespace();

    let ax = iter.read::<f64, FE>();
    let ay = iter.read::<f64, FE>();
    let az = iter.read::<f64, FE>();
    let bx = iter.read::<f64, FE>();
    let by = iter.read::<f64, FE>();
    let bz = iter.read::<f64, FE>();
    let cx = iter.read::<f64, FE>();
    let cy = iter.read::<f64, FE>();
    let cz = iter.read::<f64, FE>();

    let (cx, cy, cz) = (cx - ax, cy - ay, cz - az);
    let (bx, by, bz) = (bx - ax, by - ay, bz - az);

    let a = (0.0, 0.0, 0.0);
    let b = (bx, by, bz);
    let c = (cx, cy, cz);

    let x = (bx * cx + by * cy + bz * cz) / (bx * bx + by * by + bz * bz);

    writeln!(output, "{:.6}", if x < 0.0 || x > 1.0 {
        f64::min(distance(a, c), distance(b, c))
    } else {
        distance((x * bx, x * by, x * bz), c)
    })?;

    Ok(())
}

fn distance(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt()
}

// fn from_str_multiple<T>(s: &str) -> Result<Vec<T>, T::Err>
//     where
//         T: FromStr,
//         T::Err: Debug,
// {
//     s.split_whitespace().map(|word| word.parse()).collect()
// }