// https://www.acmicpc.net/problem/1310

use std::{
    cmp::Ordering,
    error::Error,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

type Point = (i32, i32);

fn dist(p1: &Point, p2: &Point) -> i64 {
    ((p1.0 - p2.0) as i64).pow(2) + ((p1.1 - p2.1) as i64).pow(2)
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

fn ccw_with_translated_point(a: &Point, b: &Point, c: &Point, mut d: Point) -> i32 {
    d.0 -= c.0 - b.0;
    d.1 -= c.1 - b.1;
    ccw(a, b, &d)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let n = input.trim().parse::<usize>()?;

    let mut max = 0;

    if n == 1 {
    } else {
        let mut points = (0..n)
            .map(|_| {
                input.clear();
                reader.read_line(&mut input).unwrap();
                let mut xy = input.split_ascii_whitespace();
                let x = xy.next().unwrap().parse::<i32>().unwrap();
                let y = xy.next().unwrap().parse::<i32>().unwrap();
                (x, y)
            })
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

        let mut cvxh = vec![points[0], points[1]];

        for point in &points[2..] {
            while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), point) <= 0 {
                cvxh.pop();
            }
            cvxh.push(*point);
        }

        let mut fpi = 1;

        // get cvxh
        let len = cvxh.len();
        for i in 0..len {
            while (fpi + 1) % len != i
                && ccw_with_translated_point(
                    &cvxh[i],
                    &cvxh[(i + 1) % len],
                    &cvxh[fpi % len],
                    cvxh[(fpi + 1) % len],
                ) > 0
            {
                let d = dist(&cvxh[i], &cvxh[fpi % len]);
                if max < d {
                    max = d;
                }
                fpi += 1;
            }
            let d = dist(&cvxh[i], &cvxh[fpi % len]);
            if max < d {
                max = d;
            }
        }
    }
    write!(writer, "{}", max)?;
    Ok(())
}
