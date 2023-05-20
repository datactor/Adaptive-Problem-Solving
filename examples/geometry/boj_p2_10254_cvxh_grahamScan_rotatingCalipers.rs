// https://www.acmicpc.net/problem/10254
// O(t * (n + nlogn: sorting + n: find hull + n: calipers))

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::Ordering,
};

type Point = (i32, i32);

fn dist(p1: &Point, p2: &Point) -> i64 {
    ((p1.0 - p2.0) as i64).pow(2) + ((p1.1 - p2.1) as i64).pow(2)
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let a = (p2.0 - p1.0) as i64 * (p3.1 - p1.1) as i64;
    let b = (p2.1 - p1.1) as i64 * (p3.0 - p1.0) as i64;
    if a > b { 1 }
    else if a < b { -1 }
    else { 0 }
}

fn ccw_with_translated_point(a: &Point, b: &Point, c: &Point, mut d: Point) -> i32 {
    d.0 -= c.0 - b.0;
    d.1 -= c.1 - b.1;
    ccw(a, b, &d)
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;
    let t = input.trim().parse::<usize>().unwrap();
    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input)?;
        let n = input.trim().parse::<usize>().unwrap();

        let mut points = (0..n).map(|_| {
            input.clear();
            reader.read_line(&mut input).unwrap();
            let mut xy = input.split_ascii_whitespace();
            let x = xy.next().unwrap().parse::<i32>().unwrap();
            let y = xy.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        }).collect::<Vec<Point>>();

        // Graham's Scan
        points.sort_unstable();
        let pivot = points[0];
        points[1..].sort_unstable_by(|p1, p2| {
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

        let mut cvxh = vec![points[0], points[1]];

        for point in &points[2..] {
            while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), point) <= 0 {
                cvxh.pop();
            }
            cvxh.push(*point);
        }

        let mut fpi = 1; // furthest point idx
        let mut max = 0;
        let mut p1 = cvxh[0];
        let mut p2 = cvxh[1];
        let len = cvxh.len();
        for i in 0..len {
            while (fpi + 1) % len != i &&
                ccw_with_translated_point(&cvxh[i], &cvxh[(i + 1) % len], &cvxh[fpi % len], cvxh[(fpi + 1) % len]) > 0
            {
                let d = dist(&cvxh[i], &cvxh[fpi % len]);
                if max < d {
                    p1 = cvxh[i];
                    p2 = cvxh[fpi % len];
                    max = d;
                }
                fpi += 1;
            }
            let d = dist(&cvxh[i], &cvxh[fpi % len]);
            if max < d {
                p1 = cvxh[i];
                p2 = cvxh[fpi % len];
                max = d;
            }
        }
        write!(writer, "{} {} {} {}\n", p1.0, p1.1, p2.0, p2.1).unwrap();
    }
    Ok(())
}