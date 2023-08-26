// https://www.acmicpc.net/problem/27656

use std::{
    cmp::Ordering,
    collections::HashSet,
    error::Error,
    io::{self, BufWriter, Read, Write},
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn ccw(&self, p1: &Point, p2: &Point) -> i32 {
        let a = (p1.x - self.x) as i64 * (p2.y - self.y) as i64;
        let b = (p1.y - self.y) as i64 * (p2.x - self.x) as i64;
        if a > b {
            1
        } else if a < b {
            -1
        } else {
            0
        }
    }

    fn dist_square(&self, other: &Point) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        dx * dx + dy * dy
    }

    fn is_inside(&self, layer: &Vec<Point>) -> bool {
        let left_ccw = layer[0].ccw(&layer[1], self);
        if left_ccw < 0 {
            return false;
        }
        let right_ccw = layer[0].ccw(&layer[layer.len() - 1], self);
        if right_ccw > 0 {
            return false;
        }

        let mut left = 1;
        let mut right = layer.len() - 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if layer[0].ccw(&layer[mid], self) >= 0 {
                left = mid;
            } else {
                right = mid;
            }
        }

        layer[left].ccw(self, &layer[right]) <= 0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let mut points_set = HashSet::with_capacity(n);
    for _ in 0..n {
        let x = scanner.next::<i32>()?;
        let y = scanner.next::<i32>()?;
        points_set.insert(Point { x, y });
    }

    let mut layers = Vec::new();
    while points_set.len() > 2 {
        let mut points = points_set.iter().cloned().collect::<Vec<_>>();
        let min_idx = points
            .iter()
            .enumerate()
            .min_by_key(|&(_, point)| (point.x, point.y))
            .unwrap()
            .0;
        points.swap(0, min_idx);

        let pivot = points[0];
        points[1..].sort_unstable_by(|p1, p2| {
            let c = pivot.ccw(p1, p2);
            if c > 0 {
                Ordering::Less
            } else if c < 0 {
                Ordering::Greater
            } else if pivot.dist_square(p1) < pivot.dist_square(p2) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let mut cvxh: Vec<Point> = Vec::new();
        for &next in points.iter() {
            while cvxh.len() >= 2 && cvxh[cvxh.len() - 2].ccw(cvxh.last().unwrap(), &next) <= 0 {
                cvxh.pop();
            }
            cvxh.push(next);
        }
        for p in cvxh.iter() {
            points_set.remove(p);
        }
        layers.push(cvxh);
    }

    let q = scanner.next::<usize>()?;
    for _ in 0..q {
        let x = scanner.next::<i32>()?;
        let y = scanner.next::<i32>()?;
        let query_point = Point { x, y };

        let mut score = 0;
        let mut left = 0;
        let mut right = (layers.len() - 1) as i32;

        while left <= right {
            let mid = (left + right) / 2;
            if query_point.is_inside(&layers[mid as usize]) {
                score = mid + 1;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        writeln!(writer, "{}", score)?;
    }

    Ok(())
}
