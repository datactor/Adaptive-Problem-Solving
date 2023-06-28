// https://www.acmicpc.net/problem/13310

use std::{
    io::{self, Read, Write, BufWriter, StdoutLock},
    error::Error,
    cmp::{max, Ordering},
};

type Star = (i64, i64);

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T> (&mut self) -> Result<T, Box<dyn Error>>
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

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Point {
    pos: (i32, i32),
    speed: (i32, i32),
}

impl Point {
    fn new(xp: i32, yp: i32, xs: i32, ys: i32) -> Self {
        Self {
            pos: (xp, yp),
            speed: (xs, ys),
        }
    }

    // fn moves(&mut self) {
    //     self.pos.0 += self.speed.0;
    //     self.pos.1 += self.speed.1;
    // }
}

#[derive(Debug)]
struct Points {
    vec: Vec<Point>,
}

impl Points {
    fn new() -> Self {
        Self { vec: vec![] }
    }

    fn push(&mut self, point: Point) {
        self.vec.push(point);
    }

    // fn next(&mut self) {
    //     for point in self.vec.iter_mut() {
    //         point.moves();
    //     }
    // }

    // Ternary search
    fn solve(&mut self, n: usize, t: i64, writer: &mut BufWriter<StdoutLock>) -> io::Result<()> {
        let mut s = 0;
        let mut e = t;
        let mut stars = vec![(0, 0); n];
        while s + 3 <= e {
            let l = (s + s + e) / 3;
            let r = (s + e + e) / 3;
            if max_dist_at_time(l, n, &mut stars, &self) > max_dist_at_time(r, n, &mut stars, &self) {
                s = l;
            } else {
                e = r;
            }
        }

        let mut mn = i64::MAX;
        let mut idx = s;
        for i in s..=e {
            let cur = max_dist_at_time(i, n, &mut stars, &self);
            if mn > cur {
                mn = cur;
                idx = i;
            }
        }
        write!(writer, "{}\n{}", idx, mn)
    }
}

fn max_dist_at_time(t: i64, n: usize, stars: &mut Vec<(i64, i64)>, v: &Points) -> i64 {
    for i in 0..n {
        stars[i as usize] = (v.vec[i].pos.0 as i64 + v.vec[i].speed.0 as i64 * t, v.vec[i].pos.1 as i64 + v.vec[i].speed.1 as i64 * t);
    }

    // Graham's Scan
    let min_idx = stars.iter().enumerate().min_by_key(|&(_, star)| star.0).unwrap().0;
    stars.swap(0, min_idx);
    let pivot = stars[0];
    stars[1..].sort_unstable_by(|p1, p2| {
        let c = ccw(&pivot, p1, p2);
        if c > 0 { Ordering::Less }
        else if c < 0 { Ordering::Greater }
        else if dist(&pivot, p1) < dist(&pivot, p2) { Ordering::Less }
        else { Ordering::Greater }
    });

    let mut cvxh: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &stars[i as usize]) <= 0 {
            cvxh.pop();
        }
        cvxh.push(stars[i as usize]);
    }

    // rotating calipers
    let mut max_dist = 0;
    let mut next_point_idx = 0;
    for i in 0..cvxh.len() {
        while next_point_idx + 1 < cvxh.len() && is_ccw(cvxh[i], cvxh[i + 1], cvxh[next_point_idx], cvxh[next_point_idx + 1]) {
            max_dist = max(max_dist, dist(&cvxh[i], &cvxh[next_point_idx]));
            next_point_idx += 1;
        }
        max_dist = max(max_dist, dist(&cvxh[i], &cvxh[next_point_idx]));
    }
    max_dist
}

fn is_ccw(s1: (i64, i64), e1: (i64, i64), s2: (i64, i64), e2: (i64, i64)) -> bool {
    let t1 = (e1.0 - s1.0, e1.1 - s1.1);
    let t2 = (e2.0 - s2.0, e2.1 - s2.1);
    ccw(&(0 as i64, 0 as i64), &t1, &t2) >= 0
}

fn dist(a: &Star, b: &Star) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn ccw(a: &Star, b: &Star, c: &Star) -> i64 {
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
    let (n, t) = (scanner.read::<usize>()?, scanner.read::<i64>()?);

    let mut points = Points::new();
    for _ in 0..n {
        let point = Point::new(scanner.read::<i32>()?, scanner.read::<i32>()?, scanner.read::<i32>()?, scanner.read::<i32>()?);
        points.push(point);
    }

    points.solve(n, t, &mut buf_writer)?;
    Ok(())
}