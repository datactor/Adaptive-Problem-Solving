// https://www.acmicpc.net/source/60679085

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
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    area: i32,
}

impl Triangle {
    fn new(a: Point, b: Point, c: Point) -> Self {
        let tmp =
            (
                a.x * (b.y - c.y) +
                b.x * (c.y - a.y) +
                c.x * (a.y - b.y)
            ).abs();

        let area = if tmp % 2 == 0 {
            tmp/2
        } else {
            -tmp/2
        };

        Self { a, b, c, area}
    }
}

fn ccw(a: Point, b: Point, c: Point) -> bool {
    (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y) > 0
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();

    let mut tri = Vec::with_capacity(3);

    for _ in 0..3 {
        reader.read_line(&mut input)?;
        let mut iter = input.split_ascii_whitespace();
        let p = Point::new(iter.read::<i32, IE>(), iter.read::<i32, IE>());
        tri.push(p);
        input.clear();
    }

    if ccw(tri[2], tri[1], tri[0]) {
        tri.swap(0, 2);
    }

    let triangle = Triangle::new(tri[0], tri[1], tri[2]);

    reader.read_line(&mut input)?;
    let n = input.trim().parse::<usize>().unwrap();

    let mut ans = 0;
    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input)?;
        let mut iter = input.split_ascii_whitespace();
        let p = Point::new(iter.read::<i32, IE>(), iter.read::<i32, IE>());

        if ccw(p, triangle.b, triangle.a) {
            continue;
        }
        if ccw(p, triangle.a, triangle.c) {
            continue;
        }
        if ccw(p, triangle.c, triangle.b) {
            continue;
        }

        ans += 1;
    }

    let mut decimal = 0;
    let integer = if triangle.area > 0 {
        triangle.area
    } else {
        decimal = 5;
        -triangle.area
    };

    write!(writer, "{}.{}\n{}", integer, decimal, ans)?;

    Ok(())
}