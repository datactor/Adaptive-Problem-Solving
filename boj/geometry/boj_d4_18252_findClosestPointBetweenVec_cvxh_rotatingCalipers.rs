// https://www.acmicpc.net/problem/18252

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

fn dist(a: &Point, b: &Point) -> i64 {
    ((a.0 - b.0) as i64).pow(2) + ((a.1 - b.1) as i64).pow(2)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let res = area(a, b, c, true);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

fn area(a: &Point, b: &Point, c: &Point, sign: bool) -> i64 {
    let mut res = (a.0 as i64 * b.1 as i64) + (b.0 as i64 * c.1 as i64) + (c.0 as i64 * a.1 as i64);
    res -= (b.0 as i64 * a.1 as i64) + (c.0 as i64 * b.1 as i64) + (a.0 as i64 * c.1 as i64);
    if sign {
        return res;
    }
    res.abs()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let top = (scanner.next::<i32>()?, scanner.next::<i32>()?);
    let bot = (scanner.next::<i32>()?, scanner.next::<i32>()?);
    let mut points = Vec::with_capacity(n + 2);
    points.push(top);
    points.push(bot);
    for _ in 0..n {
        let y = scanner.next::<i32>()?;
        let s = scanner.next::<i32>()?;
        let e = scanner.next::<i32>()?;
        // bot-top 벡터를 기준으로 left가 벡터보다 오른쪽에 있다면, left가 가장 가까운 점.
        // 반대로 right가 벡터보다 왼쪽에 있다면, right가 가장 가까운 점이다.
        let l = ccw(&bot, &top, &(s, y));
        let r = ccw(&bot, &top, &(e, y));
        if r == 1 {
            points.push((e, y));
        } else if l == -1 {
            points.push((s, y));
        }
    }

    let mut ans = 0;

    if points.len() > 2 {
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

        // get cvxh
        let mut cvxh: Vec<Point> = vec![];
        for i in 0..points.len() {
            while cvxh.len() >= 2
                && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &points[i]) <= 0
            {
                cvxh.pop();
            }
            cvxh.push(points[i]);
        }

        // two pointer로 cur을 구하면서, ans를 최대값으로 갱신.
        // rotating calipers를 사용해서 cvxh 위의 점들을 순회하여, 각 점에서 가장 면적이 큰 삼각형을 찾는다.
        // i는 cvxh의 시작점, j와 pt는 calipers의 끝점.
        // 각 i에 대해서 j와 pt는 cvxh를 따라 회전하면서 가장 큰 삼각형을 찾는다.
        let n = cvxh.len();
        let mut i = 0;
        let mut j = 1;
        let mut pt = 1;
        let mut cur = 0;
        while i != n {
            while i != pt {
                if area(&cvxh[i], &cvxh[j], &cvxh[(pt + 1) % n], false) < cur {
                    j = (j + 1) % n;
                } else {
                    pt = (pt + 1) % n;
                }
                cur = area(&cvxh[i], &cvxh[j], &cvxh[pt], false);
                ans = std::cmp::max(ans, cur);
            }
            i += 1;
            j = (i + 1) % n;
            pt = (j + 1) % n;
        }
    }

    if ans & 1 == 1 {
        write!(buf_writer, "{:.1}", ans as f64 / 2.0)?;
    } else {
        write!(buf_writer, "{}", ans / 2)?;
    }
    Ok(())
}
