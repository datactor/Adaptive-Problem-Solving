// https://www.acmicpc.net/problem/6850

// ref: https://darkpgmr.tistory.com/86

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
    fn new(input: &'a str) -> Self {
        Self {
            input: input.split_ascii_whitespace(),
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

fn dist(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let cross_ab_ac = a.0 * b.1 + b.0 * c.1 + c.0 * a.1;
    let cross_ba_bc = b.0 * a.1 + c.0 * b.1 + a.0 * c.1;
    if cross_ab_ac > cross_ba_bc {
        1
    } else if cross_ab_ac < cross_ba_bc {
        -1
    } else {
        0
    }
}

// cross product of Vector AB and Vector AC
fn get_triangles_area(a: &Point, b: &Point, c: &Point, half: bool) -> (i32, bool) {
    let area = ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).abs();
    if area % 2 == 0 {
        (area / 2, half)
    } else if half == true {
        (area / 2, true)
    } else {
        (area / 2, false)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let mut points = (0..n)
        .map(|_| (scanner.next().unwrap(), scanner.next().unwrap()))
        .collect::<Vec<Point>>();

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

    let mut area = 0;
    let mut half = false;

    let mut cvxh: Vec<Point> = vec![];

    if n > 2 {
        for i in 0..n {
            while cvxh.len() >= 2
                && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &points[i]) <= 0
            {
                cvxh.pop();
            }
            cvxh.push(points[i]);
        }

        for i in 1..cvxh.len() - 1 {
            let area_and_half = get_triangles_area(&cvxh[0], &cvxh[i], &cvxh[i + 1], half);
            area += area_and_half.0;

            if half && area_and_half.1 {
                area += 1;
                half = false;
            } else if area_and_half.1 {
                half = true;
            }
        }
    };
    write!(buf_writer, "{}", area / 50)?;
    Ok(())
}
