// https://www.acmicpc.net/problem/2244

use std::{
    cmp::Ordering,
    error::Error,
    io::{self, BufWriter, Read, Write},
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

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
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

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let a = (p2.0 - p1.0) as i64 * (p3.1 - p1.1) as i64;
    let b = (p2.1 - p1.1) as i64 * (p3.0 - p1.0) as i64;
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

fn dist(a: &Point, b: &Point) -> i64 {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;
    (dx * dx) + (dy * dy)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let (n, m) = (scanner.next::<usize>()?, scanner.next::<usize>()?);

    let np = (0..n)
        .map(|_| {
            (
                scanner.next::<i32>().unwrap(),
                scanner.next::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut points = Vec::with_capacity(n + m);

    for _ in 0..m {
        let x = scanner.next::<i32>()?;
        let y = scanner.next::<i32>()?;
        for (nx, ny) in np.iter() {
            points.push((x + nx, y + ny));
        }
    }

    // Graham's scana
    let min_idx = points
        .iter()
        .enumerate()
        .min_by_key(|&(_, point)| (point.0, point.1))
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

    // get cvxh
    let mut cvxh = Vec::new();
    for &next in points.iter() {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &next) <= 0 {
            cvxh.pop();
        }
        cvxh.push(next);
    }

    writeln!(writer, "{}", cvxh.len())?;
    for i in cvxh {
        writeln!(writer, "{} {}", i.0, i.1)?;
    }
    Ok(())
}
