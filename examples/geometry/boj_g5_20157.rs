// https://www.acmicpc.net/problem/20157

use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    num::{ParseIntError as IE, ParseFloatError as FE},
    fmt,
    collections::HashMap,
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
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Line {
    slope: u64,
    quadrant: u8,
}

impl Line {
    fn new(p: Point) -> Self {
        let x = p.x as f64;
        let y = p.y as f64;
        let mut slope = (y/x).to_bits();
        let q = match (p.x.signum(), p.y.signum()) {
            (1, 1) => 1,
            (1, -1) => 2,
            (-1, -1) => 3,
            (-1, 1) => 4,
            (0, 1) => {
                slope = f64::INFINITY.to_bits();
                5
            },
            (0, -1) => {
                slope = f64::INFINITY.to_bits();
                6
            },
            (1, 0) => 7,
            (-1, 0) => 8,
            _ => 0,
        };
        Self {
            slope,
            quadrant: q,
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input)?;

    let n = input.trim().parse::<usize>().unwrap();

    let mut balloons = HashMap::with_capacity(n);

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input)?;
        let mut iter = input.split_ascii_whitespace();
        let (x, y) = (iter.read::<i32, IE>(), iter.read::<i32, IE>());
        let p = Point::new(x, y);
        let line = Line::new(p);
        let val = balloons.entry((line.slope, line.quadrant)).or_insert(0);
        *val += 1;
    }

    writeln!(writer, "{}", balloons.values().max().unwrap())?;

    Ok(())
}