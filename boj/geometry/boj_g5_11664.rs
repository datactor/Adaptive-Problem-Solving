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
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x, y, z
        }
    }

    fn distance(&self, other: &Point) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32).sqrt()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Line(Point, Point);

impl Line {
    fn min_distance_to_line(&self, point: &Point) -> f32 {
        let x = ((self.1.x - self.0.x) * (point.x - self.0.x) + (self.1.y - self.0.y) * (point.y - self.0.y) + (self.1.z - self.0.z) * (point.z - self.0.z)) as f32
            / ((self.1.x - self.0.x).pow(2) + (self.1.y - self.0.y).pow(2) + (self.1.z - self.0.z).pow(2)) as f32;

        if x < 0.0 || x > 1.0 {
            f32::min(point.distance(&self.0), point.distance(&self.1))
        } else {
            let px = (self.0.x as f32 + x * (self.1.x - self.0.x) as f32) as f32;
            let py = (self.0.y as f32 + x * (self.1.y - self.0.y) as f32) as f32;
            let pz = (self.0.z as f32 + x * (self.1.z - self.0.z) as f32) as f32;
            ((point.x as f32 - px).powf(2.0) + (point.y as f32 - py).powf(2.0) + (point.z as f32 - pz).powf(2.0)).sqrt()
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