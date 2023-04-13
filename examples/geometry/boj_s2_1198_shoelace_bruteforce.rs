// https://www.acmicpc.net/problem/1198
// variation of the shoelace formula

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main () -> io::Result<()> {
    let mut input = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(io::stdout().lock());
    let mut line = || input.next().unwrap();
    let n = line().parse::<usize>().unwrap();

    let mut read = || {
        let line = line();
        let (a, b) = line.split_once(' ').unwrap();
        (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    };
    let v = (0..n).map(|_| read()).collect::<Vec<(i32, i32)>>();

    let mut max: f64 = 0.0;
    for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                max = max.max(triangle_area(v[i], v[j], v[k]));
            }
        }
    }

    writeln!(output, "{:0.9}", max)?;
    Ok(())
}

fn triangle_area(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> f64 {
    let abx = b.0 - a.0;
    let aby = b.1 - a.1;
    let acx = c.0 - a.0;
    let acy = c.1 - a.1;
    (abx * acy - aby * acx).abs() as f64 / 2.0
}