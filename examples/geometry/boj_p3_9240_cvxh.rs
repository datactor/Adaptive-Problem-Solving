// https://www.acmicpc.net/problem/9240

use std::{
    io::{self, Write, BufRead, BufWriter},
    cmp::Ordering,
    f64,
};

type Point = (i32, i32);

fn dist(p1: Point, p2: Point) -> i32 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let c: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(c);

    for is_line in lines {
        let line = is_line?;
        let xy = line.split_once(' ').unwrap();
        let x = xy.0.parse::<i32>().unwrap();
        let y = xy.1.parse::<i32>().unwrap();
        points.push((x, y));
    }

    points.sort();
    let pivot = points[0];
    points[1..].sort_by(|p1, p2| {
        let c = ccw(&pivot, p1, p2);
        if c > 0 {
            return Ordering::Less;
        }
        if c < 0 {
            return Ordering::Greater;
        }
        if dist(pivot, *p1) < dist(pivot, *p2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut cvxh: Vec<Point> = Vec::new();
    cvxh.push(points[0]);
    cvxh.push(points[1]);

    for i in 2..c {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &points[i]) <= 0 {
            cvxh.pop();
        }
        cvxh.push(points[i]);
    }

    let mut max: i32 = 0;
    let len = cvxh.len();
    for i in 0..len {
        for j in i+1..len {
            max = max.max(dist(cvxh[i], cvxh[j]));
        }
    }

    write!(BufWriter::new(io::stdout().lock()), "{:.6}", (max as f64).sqrt())?;
    Ok(())
}