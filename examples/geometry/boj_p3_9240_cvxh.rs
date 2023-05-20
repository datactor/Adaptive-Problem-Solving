// https://www.acmicpc.net/problem/9240

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::Ordering,
};

type Point = (i32, i32);

fn dist(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    (p2.0 - p1.0) * (p3.1 - p1.1) - (p2.1 - p1.1) * (p3.0 - p1.0)
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut input = String::new();
    reader.read_line(&mut input)?;

    let c: usize = input.trim().parse::<usize>().unwrap();

    let mut points = (0..c).map(|_| {
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
        let ord = ccw(&pivot, p1, p2);
        if ord > 0 {
            Ordering::Less
        } else if ord < 0 {
            Ordering::Greater
        } else if dist(&pivot, p1) < dist(&pivot, p2) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut cvxh: Vec<Point> = vec![points[0], points[1]];
    for i in 2..c {
        while cvxh.len() >= 2 && ccw(&cvxh[cvxh.len() - 2], cvxh.last().unwrap(), &points[i]) <= 0 {
            cvxh.pop();
        }
        cvxh.push(points[i]);
    }

    let mut max = 0;
    let mut fpi = 1;
    let len = cvxh.len();

    // for i in 0..len {
    //     for j in i+1..len {
    //         max = max.max(dist(cvxh[i], cvxh[j]));
    //     }
    // }

    // rotate calipers
    for i in 0..len {
        while fpi + 1 != i &&
            ccw(&cvxh[i], &cvxh[(i + 1) % len], &cvxh[(fpi + 1) % len]) > ccw(&cvxh[i], &cvxh[(i + 1) % len], &cvxh[fpi])
        {
            max = max.max(dist(&cvxh[i], &cvxh[fpi]));
            fpi = if fpi + 1 < len { fpi + 1 } else { 0 };
        }
        max = max.max(dist(&cvxh[i], &cvxh[fpi]));
    }

    write!(BufWriter::new(io::stdout().lock()), "{:.6}", (max as f64).sqrt())?;
    Ok(())
}