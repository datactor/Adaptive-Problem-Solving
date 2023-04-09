// https://www.acmicpc.net/problem/11662

use std::io::{self, prelude::*, BufWriter};

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn interpolate(&self, other: &Self, t: f64) -> Self {
        let x = self.x + (other.x - self.x) * t;
        let y = self.y + (other.y - self.y) * t;
        Self::new(x, y)
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let v = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<_>>();

    let a_start = Point::new(v[0], v[1]);
    let a_end = Point::new(v[2], v[3]);
    let b_start = Point::new(v[4], v[5]);
    let b_end = Point::new(v[6], v[7]);

    // ternary search
    let mut lo = 0.0;
    let mut hi = 1.0;
    let mut ans = f64::INFINITY;

    while hi - lo >= 1e-8 {
        let p = (2.0 * lo + hi) / 3.0;
        let q = (lo + 2.0 * hi) / 3.0;

        let a = a_start.interpolate(&a_end, p);
        let b = b_start.interpolate(&b_end, p);
        let c = a_start.interpolate(&a_end, q);
        let d = b_start.interpolate(&b_end, q);

        let distance_p = a.distance(&b);
        let distance_q = c.distance(&d);

        ans = ans.min(distance_p.min(distance_q));

        if distance_p < distance_q {
            hi = q;
        } else {
            lo = p;
        }
    }

    writeln!(output, "{:.6}", ans)?;

    Ok(())
}
