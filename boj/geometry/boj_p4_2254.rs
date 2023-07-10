// https://www.acmicpc.net/problem/2254

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    cmp::Ordering,
};

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
        self.input.next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    cvxh: bool,
    idx: usize,
}

impl Point {
    fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
        let a = (p2.x - p1.x) as i64 * (p3.y - p1.y) as i64;
        let b = (p2.y - p1.y) as i64 * (p3.x - p1.x) as i64;
        if a > b { 1 }
        else if a < b { -1 }
        else { 0 }
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
    let mut scanner = Scanner::new(&buffer);
    let (n, px, py): (usize, i32, i32) = (scanner.next()?, scanner.next()?, scanner.next()?);

    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let x = scanner.next::<i32>()?;
        let y = scanner.next::<i32>()?;
        points.push(Point { x, y, cvxh: false, idx: i });
    }
    
    let mut cnt = 0;
    let prison = Point { x: px, y: py, cvxh: false, idx: 100_001 };

    'round: while points.len() > 2 {
        // Graham's scan
        let min_idx = points.iter().enumerate().min_by_key(|&(_, point)| point.x).unwrap().0;
        points.swap(0, min_idx);

        let pivot = points[0];
        points[1..].sort_unstable_by(|p1, p2| {
            let c = Point::ccw(&pivot, p1, p2);
            if c > 0 { Ordering::Less }
            else if c < 0 { Ordering::Greater }
            else if Point::dist(&pivot, p1) < Point::dist(&pivot, p2) { Ordering::Less }
            else { Ordering::Greater }
        });

        // get cvxh
        let mut cvxh = Vec::new();
        for i in 0..points.len() {
            let point = points[i];
            while cvxh.len() >= 2 && Point::ccw(&cvxh[cvxh.len()-2], cvxh.last().unwrap(), &point) <= 0 {
                cvxh.pop();
            }
            points[i].cvxh = true;
            cvxh.push(point);
        }
        cvxh.push(cvxh[0]);

        // remove points cvxh edges
        // points = points.into_iter().filter(|p| !p.cvxh).collect();

        // Todo!(Reduce complexity of removing cvxh edges from O(n^2) to O(n)
        points = points.into_iter().filter(|p| !cvxh.contains(p)).collect();
        
        // check if prison is inside cvxh
        for i in 0..cvxh.len()-1 {
            if Point::ccw(&cvxh[i], &cvxh[i+1], &prison) <= 0 {
                break 'round;
            }
        }
        cnt += 1;
    }

    write!(buf_writer, "{}", cnt)?;
    Ok(())
}