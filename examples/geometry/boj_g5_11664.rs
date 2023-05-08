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

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Line(Point, Point);

impl Line {
    fn min_distance_to_line(&self, point: &Point) -> f64 {
        let x = ((self.1.x - self.0.x) * (point.x - self.0.x) + (self.1.y - self.0.y) * (point.y - self.0.y) + (self.1.z - self.0.z) * (point.z - self.0.z))
            / ((self.1.x - self.0.x).powi(2) + (self.1.y - self.0.y).powi(2) + (self.1.z - self.0.z).powi(2));

        if x < 0.0 || x > 1.0 {
            f64::min(point.distance(&self.0), point.distance(&self.1))
        } else {
            let projected_point = Point::new(self.0.x + x * (self.1.x - self.0.x), self.0.y + x * (self.1.y - self.0.y), self.0.z + x * (self.1.z - self.0.z));
            point.distance(&projected_point)
        }
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_ascii_whitespace();

    let line = Line(
        Point::new(iter.read(), iter.read(), iter.read()),
        Point::new(iter.read(), iter.read(), iter.read()),
    );
    let point = Point::new(iter.read(), iter.read(), iter.read());

    let ans = line.min_distance_to_line(&point);

    writeln!(output, "{:.6}", ans)?;

    Ok(())
}

// fn from_str_multiple<T>(s: &str) -> Result<Vec<T>, T::Err>
//     where
//         T: FromStr,
//         T::Err: Debug,
// {
//     s.split_whitespace().map(|word| word.parse()).collect()
// }