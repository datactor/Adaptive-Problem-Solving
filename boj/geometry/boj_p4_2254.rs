// https://www.acmicpc.net/problem/2254

use std::{
    io::{self, Read, Write, BufWriter},
    error::Error,
    cmp::Ordering,
    collections::HashSet,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn ccw(t1: &Point, t2: &Point, t3: &Point) -> bool {
        let s = (t2.x - t1.x) * (t3.y - t1.y) - (t2.y - t1.y) * (t3.x - t1.x);
        s > 0
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf_writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let (n, px, py): (usize, i64, i64) = (scanner.next()?, scanner.next()?, scanner.next()?);

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let x = scanner.next::<i64>()?;
        let y = scanner.next::<i64>()?;
        points.push(Point { x, y });
    }
    
    let mut cnt = 0;
    let pxy = Point { x: px, y: py };
    let mut is_hull = true;

    while is_hull && points.len() > 2 {
        points.sort_unstable();
        let mut bot_cvxh = vec![points[0], points[1]];
        let mut top_cvxh = vec![*points.last().unwrap(), points[points.len()-2]];

        for point in points[2..].iter() {
            bot_cvxh.push(*point);
            let mut p = true;
            while p && bot_cvxh.len() > 2 {
                let p1 = bot_cvxh.pop().unwrap();
                let p2 = bot_cvxh.pop().unwrap();
                if Point::ccw(&bot_cvxh.last().unwrap(), &p2, &p1) {
                    bot_cvxh.push(p2);
                    bot_cvxh.push(p1);
                    p = false;
                } else {
                    bot_cvxh.push(p1);
                }
            }
        }

        for point in points[..points.len()-2].iter().rev() {
            top_cvxh.push(*point);
            let mut p = true;
            while p && top_cvxh.len() > 2 {
                let p1 = top_cvxh.pop().unwrap();
                let p2 = top_cvxh.pop().unwrap();
                if Point::ccw(&top_cvxh.last().unwrap(), &p2, &p1) {
                    top_cvxh.push(p2);
                    top_cvxh.push(p1);
                    p = false;
                } else {
                    top_cvxh.push(p1);
                }
            }
        }

        bot_cvxh.pop();
        let cvxh = [bot_cvxh.clone(), top_cvxh.clone()].concat();

        let cvxh_set: HashSet<_> = cvxh.iter().cloned().collect();
        let points_set: HashSet<_> = points.iter().cloned().collect();
        points = points_set.difference(&cvxh_set).copied().collect();

        for i in 0..cvxh.len()-1 {
            if !Point::ccw(&cvxh[i], &cvxh[i+1], &pxy) {
                is_hull = false;
                break;
            }
        }

        if is_hull {
            cnt += 1;
        }
    }

    write!(buf_writer, "{}", cnt)?;
    Ok(())
}
