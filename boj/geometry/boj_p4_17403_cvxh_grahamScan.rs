// https://www.acmicpc.net/problem/17403

use std::{
    cmp::Ordering,
    error::Error,
    io::{self, BufWriter, Read, Write},
};

struct Scanner<'a> {
    reader: Box<dyn Read + 'a>,
    buf: Vec<u8>,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new<T: Read + 'a>(reader: T) -> Self {
        Self {
            reader: Box::new(reader),
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
            self.reader.read_to_end(&mut self.buf).expect("read fail");
            self.pos = 0;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    i: usize,
}

impl Point {
    fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
        let a = (p2.x - p1.x) as i64 * (p3.y - p1.y) as i64;
        let b = (p2.y - p1.y) as i64 * (p3.x - p1.x) as i64;
        if a > b {
            1
        } else if a < b {
            -1
        } else {
            0
        }
    }

    fn dist(a: &Point, b: &Point) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        (dx * dx) as i64 + (dy * dy) as i64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(buffer.as_bytes());
    let n = scanner.next::<usize>()?;
    let mut result = vec![0; n];
    let mut points = (0..n)
        .map(|i| {
            let x = scanner.next::<i32>()?;
            let y = scanner.next::<i32>()?;
            Ok(Point { x, y, i })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let mut layer = 1;

    while !points.is_empty() {
        // Graham's scan
        let min_idx = points
            .iter()
            .enumerate()
            .min_by_key(|&(_, point)| (point.x, point.y))
            .unwrap()
            .0;
        points.swap(0, min_idx);

        let pivot = points[0];
        points[1..].sort_unstable_by(|p1, p2| {
            let c = Point::ccw(&pivot, p1, p2);
            if c > 0 {
                Ordering::Less
            } else if c < 0 {
                Ordering::Greater
            } else if Point::dist(&pivot, p1) < Point::dist(&pivot, p2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        // get cvxh
        let mut cvxh = Vec::new();
        for &next in points.iter() {
            while cvxh.len() >= 2
                && Point::ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &next) <= 0
            {
                cvxh.pop();
            }
            cvxh.push(next);
        }

        // get layer index of cvxh point
        if cvxh.len() <= 2 {
            for point in &cvxh {
                result[point.i] = 0;
            }
            break;
        } else {
            for point in &cvxh {
                result[point.i] = layer;
            }
            points.retain(|point| !cvxh.contains(point));
            layer += 1;
        }
    }

    for i in result {
        write!(buf_writer, "{} ", i)?;
    }
    Ok(())
}
