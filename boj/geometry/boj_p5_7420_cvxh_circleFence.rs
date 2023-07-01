// https://www.acmicpc.net/problem/7420

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    cmp::Ordering,
};

type Point = (i32, i32);

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

fn dist(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let cross_ab_ac = a.0 * b.1 + b.0 * c.1 + c.0 * a.1;
    let cross_ba_bc = b.0 * a.1 + c.0 * b.1 + a.0 * c.1;
    if cross_ab_ac > cross_ba_bc { 1 }
    else if cross_ab_ac < cross_ba_bc { -1 }
    else { 0 }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let l = scanner.next::<f32>()?;
    let mut points = (0..n).map(|_| (scanner.next().unwrap(), scanner.next().unwrap())).collect::<Vec<Point>>();

    // Graham's Scan
    let min_idx = points.iter().enumerate().min_by_key(|&(_, (x, _))| x).unwrap().0;
    points.swap(0, min_idx);
    let pivot = points[0];
    points[1..].sort_unstable_by(|p1, p2| {
        let c = ccw(&pivot, p1, p2);
        if c > 0 { Ordering::Less }
        else if c < 0 { Ordering::Greater }
        else if dist(&pivot, p1) < dist(&pivot, p2) { Ordering::Less }
        else { Ordering::Greater }
    });

    let mut cvxh: Vec<Point> = vec![];
    for i in 0..n {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &points[i]) <= 0 {
            cvxh.pop();
        }
        cvxh.push(points[i]);
    }
    cvxh.push(cvxh[0]);

    let mut sum = 2.0 * l * std::f32::consts::PI;
    for i in 1..cvxh.len() {
        sum += (dist(&cvxh[i-1], &cvxh[i]) as f32).sqrt()
    }
    write!(buf_writer, "{:0.0}", sum)?;
    Ok(())
}