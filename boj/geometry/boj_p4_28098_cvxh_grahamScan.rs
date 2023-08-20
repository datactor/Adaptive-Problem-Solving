// https://www.acmicpc.net/problem/28098

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
            .ok_or("EOF")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

fn dist_square(a: &Point, b: &Point) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
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

fn is_on_segment(pivot: &Point, a: &Point, b: &Point) -> bool {
    ccw(pivot, a, b) == 0
        && pivot.0 >= a.0.min(b.0)
        && pivot.0 <= a.0.max(b.0)
        && pivot.1 >= a.1.min(b.1)
        && pivot.1 <= a.1.max(b.1)
}

fn cross_x(pivot: &Point, a: &Point, b: &Point) -> f64 {
    let numerator = (b.0 - a.0) as f64 * (pivot.1 - a.1) as f64;
    let denominator = (b.1 - a.1) as f64;
    numerator / denominator + a.0 as f64
}

fn is_inside(pivot: &Point, polygon: &Vec<Point>) -> bool {
    let mut cross_cnt = 0;
    let len = polygon.len();
    for i in 0..len {
        let j = (i + 1) % len;

        if is_on_segment(pivot, &polygon[i], &polygon[j]) {
            return false;
        }

        if (polygon[i].1 > pivot.1) != (polygon[j].1 > pivot.1) {
            if (pivot.0 as f64) < cross_x(pivot, &polygon[i], &polygon[j]) {
                cross_cnt += 1;
            }
        }
    }
    cross_cnt % 2 > 0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    if n < 3 {
        writeln!(writer, "Yes")?;
        return Ok(());
    } else {
        let mut points = (0..n)
            .map(|_| {
                (
                    scanner.next::<i32>().unwrap(),
                    scanner.next::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

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
            } else if dist_square(&pivot, p1) < dist_square(&pivot, p2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut cvxh = Vec::new();
        for &next in points.iter() {
            while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &next) <= 0 {
                cvxh.pop();
            }
            cvxh.push(next);
        }

        writeln!(
            writer,
            "{}",
            if is_inside(&(0, 0), &cvxh) {
                "No"
            } else {
                "Yes"
            }
        )?;
    }
    Ok(())
}
