use std::{
    io::{self, prelude::*, BufReader, BufWriter},
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

enum TriangleType {
    Equilateral,
    Isosceles,
    Scalene,
    Error,
}

impl fmt::Display for TriangleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriangleType::Equilateral => write!(f, "Equilateral"),
            TriangleType::Isosceles => write!(f, "Isosceles"),
            TriangleType::Scalene => write!(f, "Scalene"),
            TriangleType::Error => write!(f, "Error"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut iter = input.split_ascii_whitespace();
    let (a, b, c) = (
        iter.read::<usize, IE>(),
        iter.read::<usize, IE>(),
        iter.read::<usize, IE>(),
    );

    let triangle_type = if a + b + c == 180 {
        if a == 60 && b == 60 && c == 60 {
            TriangleType::Equilateral
        } else if a == b || a == c || b == c {
            TriangleType::Isosceles
        } else {
            TriangleType::Scalene
        }
    } else {
        TriangleType::Error
    };

    writeln!(writer, "{}", triangle_type)?;

    Ok(())
}