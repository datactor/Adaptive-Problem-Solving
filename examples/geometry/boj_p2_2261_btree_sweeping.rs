// https://www.acmicpc.net/problem/2261
// O(2 * n log n + n)

use std::{
    io::{self, Write, BufRead, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    fmt,
    collections::BTreeSet,
};

type Point = (i32, i32);

trait Parser {
    fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
}

impl<'a> Parser for SplitAsciiWhitespace<'a> {
    fn read<T, E>(&mut self) -> T
        where
            T: FromStr<Err = E>,
            E: fmt::Debug,
    {
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

fn dist(p1: &Point, p2: &Point) -> i32 {
    (p2.1 - p1.1).pow(2) + (p2.0 - p1.0).pow(2)
}

fn main() -> io::Result<()> {
    let mut n = 0;
    let mut points: Vec<Point> = Vec::with_capacity(0);

    for res_line in io::stdin().lock().lines() {
        let line = res_line?;
        let mut iter = line.split_ascii_whitespace();
        let x: i32 = iter.read();
        if let Some(val) = iter.next() {
            let y = val.parse::<i32>().unwrap();
            points.push((x, y));
        } else {
            n = x as usize;
            points = Vec::with_capacity(n);
        };
    }

    points.sort();
    let mut btset: BTreeSet<Point> = BTreeSet::new();
    btset.insert((points[0].1, points[0].0));
    btset.insert((points[1].1, points[1].0));

    let mut min = dist(&points[0], &points[1]);
    let mut idx = 0;

    for i in 2..n {
        while idx < i {
            let d = points[i].0 - points[idx].0;
            if d * d <= min {
                break;
            } else {
                btset.remove(&(points[idx].1, points[idx].0));
                idx += 1;
            }
        }

        let lower_bound = (points[i].1 - (min as f64).sqrt() as i32, i32::MIN);
        let upper_bound = (points[i].1 + (min as f64).sqrt() as i32, i32::MAX);

        let range = btset.range(lower_bound..=upper_bound);
        for p in range {
            min = min.min(dist(&(p.1, p.0), &points[i]));
        }
        btset.insert((points[i].1, points[i].0));
    }

    writeln!(BufWriter::new(io::stdout().lock()), "{:?}", min)?;
    Ok(())
}