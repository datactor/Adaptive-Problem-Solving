// https://www.acmicpc.net/problem/2049

use std::{
    cmp::Ordering,
    error::Error,
    io::{self, Write, Read, BufWriter},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>
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
        self.input
            .next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

type Point = (i32, i32);

fn dist(p1: &Point, p2: &Point) -> i64 {
    ((p1.0 - p2.0) as i64).pow(2) + ((p1.1 - p2.1) as i64).pow(2)
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let a = (p2.0 - p1.0) as i64 * (p3.1 - p1.1) as i64;
    let b = (p2.1 - p1.1) as i64 * (p3.0 - p1.0) as i64;
    if a > b { 1 }
    else if a < b { -1 }
    else { 0 }
}

fn ccw_with_translated_point(a: &Point, b: &Point, c: &Point, mut d: Point) -> i32 {
    d.0 -= c.0 - b.0;
    d.1 -= c.1 - b.1;
    ccw(a, b, &d)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let mut points = (0..n)
        .map(|_| (scanner.next::<i32>().unwrap(), scanner.next::<i32>().unwrap()))
        .collect::<Vec<Point>>();

    let mut max = 0;

    // Graham's Scan
    let min_idx = points
        .iter()
        .enumerate()
        .min_by_key(|&(_, (x, _))| x)
        .unwrap()
        .0;
    points.swap(0, min_idx);
    let pivot = points[0];
    points[1..].sort_unstable_by(|p1, p2| {
        let c = ccw(&pivot, p1, p2);
        if c > 0 {
            Ordering::Less
        } else if c < 0 {
            Ordering::Greater
        } else if dist(&pivot, p1) < dist(&pivot, p2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut cvxh = vec![points[0], points[1]];

    for point in &points[2..] {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), point) <= 0 {
            cvxh.pop();
        }
        cvxh.push(*point);
    }

    let mut fpi = 1;

    // get cvxh
    let len = cvxh.len();
    for i in 0..len {
        while (fpi + 1) % len != i
            && ccw_with_translated_point(
            &cvxh[i],
            &cvxh[(i + 1) % len],
            &cvxh[fpi % len],
            cvxh[(fpi + 1) % len],
        ) > 0
        {
            let d = dist(&cvxh[i], &cvxh[fpi % len]);
            if max < d {
                max = d;
            }
            fpi += 1;
        }
        let d = dist(&cvxh[i], &cvxh[fpi % len]);
        if max < d {
            max = d;
        }
    }
    write!(buf_writer, "{}", max)?;
    Ok(())
}