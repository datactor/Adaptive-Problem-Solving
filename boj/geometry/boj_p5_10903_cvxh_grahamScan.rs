use std::{
    cmp::Ordering,
    error::Error,
    io::{self, BufWriter, Read, Write},
};

type Point = (i32, i32);

struct Scanner<'a> {
    reader: Box<dyn io::Read + 'a>,
    buf: Vec<u8>,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(reader: Box<dyn io::Read + 'a>) -> Self {
        Self {
            reader,
            buf: Vec::new(),
            pos: 0,
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> Result<T, Box<dyn Error>> {
        loop {
            if let Some(i) = self.buf[self.pos..]
                .iter()
                .position(|&c| c == b' ' || c == b'\n')
            {
                let res = std::str::from_utf8(&self.buf[self.pos..self.pos + i])
                    .unwrap()
                    .parse::<T>()
                    .ok()
                    .expect("parse fail");
                self.pos += i + 1;
                return Ok(res);
            }
            self.buf.clear();
            self.reader.read_to_end(&mut self.buf)?;
            self.pos = 0;
        }
    }
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i64 {
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
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx) as i64 + (dy * dy) as i64
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(Box::new(buffer.as_bytes()));
    let (n, r) = (scanner.next::<usize>()?, scanner.next::<f64>()?);
    let mut points = (0..n)
        .map(|_| {
            (
                scanner.next::<i32>().unwrap(),
                scanner.next::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // Graham's scan
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
    cvxh.push(cvxh[0]);

    let mut sum = 2.0 * r * std::f64::consts::PI;
    for i in 1..cvxh.len() {
        sum += (dist(&cvxh[i - 1], &cvxh[i]) as f64).sqrt()
    }
    write!(writer, "{:.8}", sum)?;
    Ok(())
}
