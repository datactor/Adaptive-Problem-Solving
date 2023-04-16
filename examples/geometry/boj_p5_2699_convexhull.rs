// https://www.acmicpc.net/problem/2699

use std::{
    io::{self, prelude::*, BufWriter},
    cmp::Ordering,
};

type Point = (i64, i64);
type Polygon = Vec<Point>;

fn point_minus(a: Point, b: Point) -> Point {
    (a.0 - b.0, a.1 - b.1)
}

fn norm(a: Point) -> f64 {
    ((a.0.pow(2) + a.1.pow(2)) as f64).sqrt()
}

fn ccw(a: Point, b: Point) -> i64 {
    (a.0 * b.1 - a.1 * b.0) as i64
}

fn ccw3(p: Point, a: Point, b: Point) -> i64 {
    let a = point_minus(a, p);
    let b = point_minus(b, p);
    ccw(a, b)
}

fn cmp(a: &Point, b: &Point) -> Ordering {
    if a.1 > b.1 {
        Ordering::Less
    } else if a.1 == b.1 {
        if a.0 < b.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Greater
    }
}

fn gift_wrap(points: &mut Polygon) -> Polygon {
    let n = points.len();

    points.sort_by(cmp);
    let pivot = points[0];

    let mut hull = vec![pivot];

    loop {
        let ph = *hull.last().unwrap();
        let mut next = points[0];

        for i in 1..n {
            let cross = ccw3(ph, next, points[i]);
            let dist = norm(point_minus(next, ph)) - norm(point_minus(points[i], ph));

            if cross > 0 || (cross == 0 && dist < 0.0) {
                next = points[i];
            }
        }

        if next == pivot {
            break;
        }

        hull.push(next);
    }

    hull
}

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines().map(Result::unwrap);
    let mut output = BufWriter::new(io::stdout().lock());
    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = input.next().unwrap().parse().unwrap();
        let mut points = Vec::new();

        for _ in 0..n / 5 {
            let data: Vec<i64> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for i in 0..5 {
                points.push((data[2 * i], data[2 * i + 1]));
            }
        }

        if n % 5 != 0 {
            let data: Vec<i64> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for i in 0..(n % 5) {
                points.push((data[2 * i], data[2 * i + 1]));
            }
        }

        let hull = gift_wrap(&mut points);
        writeln!(output, "{}", hull.len())?;
        for (x, y) in hull {
            writeln!(output, "{} {}", x, y)?;
        }
    }
    Ok(())
}