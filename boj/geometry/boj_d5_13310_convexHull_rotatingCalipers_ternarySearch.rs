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
    fn solve(&mut self, n: i64, t: i64, writer: &mut BufWriter<StdoutLock>) -> io::Result<()> {
        let mut s = 0;
        let mut e = t;
        let mut stars = vec![(0, 0); n as usize];
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

fn max_dist_at_time(t: i64, n: i64, stars: &mut Vec<(i64, i64)>, v: &Points) -> i64 {
    for i in 0..n {
        stars[i as usize] = (v.vec[i as usize].pos.0 as i64 + v.vec[i as usize].speed.0 as i64 * t, v.vec[i as usize].pos.1 as i64 + v.vec[i as usize].speed.1 as i64 * t);
    }

    // Graham's Scan
    let min_idx = stars.iter().enumerate().min_by_key(|&(_, star)| star.0).unwrap().0;
    stars.swap(0, min_idx);
    let pivot = stars[0];
    stars[1..].sort_unstable_by(|p1, p2| {
        let c = ccw(&pivot, p1, p2);
        if c > 0 {
            return Ordering::Less;
        }
        if c < 0 {
            return Ordering::Greater;
        }
        if dist(&pivot, p1) < dist(&pivot, p2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut cvxh: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &stars[i as usize]) <= 0 {
            cvxh.pop();
        }
        cvxh.push(stars[i as usize]);
    }

    let mut ret = 0;
    let mut p = 0;
    for i in 0..cvxh.len() {
        while p + 1 < cvxh.len() && is_ccw(cvxh[i], cvxh[i + 1], cvxh[p], cvxh[p + 1]) {
            ret = max(ret, dist(&cvxh[i], &cvxh[p]));
            p += 1;
        }
        ret = max(ret, dist(&cvxh[i], &cvxh[p]));
    }
    ret
}

fn is_ccw(s1: (i64, i64), e1: (i64, i64), s2: (i64, i64), e2: (i64, i64)) -> bool {
    let t1 = (e1.0 - s1.0, e1.1 - s1.1);
    let t2 = (e2.0 - s2.0, e2.1 - s2.1);
    ccw(&(0 as i64, 0 as i64), &t1, &t2) >= 0
}

fn dist(a: &Star, b: &Star) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}

fn ccw(a: &Star, b: &Star, c: &Star) -> i64 {
    let res1 = a.0 * b.1 + b.0 * c.1 + c.0 * a.1;
    let res2 = b.0 * a.1 + c.0 * b.1 + a.0 * c.1;
    if res1 > res2 { 1 }
    else if res1 < res2 { -1 }
    else { 0 }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);
    let (n, t) = (scanner.read::<i64>()?, scanner.read::<i64>()?);

    let mut points = Points::new();
    for _ in 0..n {
        let point = Point::new(scanner.read::<i32>()?, scanner.read::<i32>()?, scanner.read::<i32>()?, scanner.read::<i32>()?);
        points.push(point);
    }

    points.solve(n, t, &mut buf_writer)?;
    // 삼분탐색을 쓰지 않음
    // for i in 0..t+1 {
    //     if i != 0 {
    //         points.next();
    //     }
    //
    //     // println!("i: {}, points: {:?}", i, points);
    //     // Graham's Scan
    //     points.vec.sort_unstable();
    //     let pivot = points.vec[0];
    //     points.vec[1..].sort_unstable_by(|p1, p2| {
    //         let c = ccw(&pivot, p1, p2);
    //         if c > 0 {
    //             return Ordering::Less;
    //         }
    //         if c < 0 {
    //             return Ordering::Greater;
    //         }
    //         if dist(&pivot, p1) < dist(&pivot, p2) {
    //             Ordering::Less
    //         } else {
    //             Ordering::Greater
    //         }
    //     });
    //
    //
    //     let mut cvxh = vec![points.vec[0], points.vec[1]];
    //
    //     for point in &points.vec[2..] {
    //         while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), point) <= 0 {
    //             cvxh.pop();
    //         }
    //         cvxh.push(*point);
    //     }
    //
    //     let mut fpi = 1; // furthest point idx
    //     let mut cur_max = 0;
    //     let mut p1 = cvxh[0];
    //     let mut p2 = cvxh[1];
    //     let len = cvxh.len();
    //     for i in 0..len {
    //         while (fpi + 1) % len != i &&
    //             ccw_with_translated_point(&cvxh[i], &cvxh[(i + 1) % len], &cvxh[fpi % len], cvxh[(fpi + 1) % len]) > 0
    //         {
    //             let d = dist(&cvxh[i], &cvxh[fpi % len]);
    //             if cur_max < d {
    //                 p1 = cvxh[i];
    //                 p2 = cvxh[fpi % len];
    //                 cur_max = d;
    //             }
    //             fpi += 1;
    //         }
    //         let d = dist(&cvxh[i], &cvxh[fpi % len]);
    //         if cur_max < d {
    //             p1 = cvxh[i];
    //             p2 = cvxh[fpi % len];
    //             cur_max = d;
    //         }
    //     }
    //     // println!("{}", cur_max);
    //     if mn_date.0 > cur_max {
    //         mn_date = (cur_max, i);
    //     }
    // }
    //
    // write!(buf_writer, "{}\n{}", mn_date.1, mn_date.0)?;
    Ok(())
}