// https://www.acmicpc.net/problem/1485
// O(6t)

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut line = || input.next().unwrap().unwrap();
    let mut read = || line().split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let t = read()[0];
    for _ in 0..t {
        let a = read();
        let b = read();
        let c = read();
        let d = read();

        let mut edges = [0; 6];

        edges[0] = (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2);
        edges[1] = (a[0] - c[0]).pow(2) + (a[1] - c[1]).pow(2);
        edges[2] = (a[0] - d[0]).pow(2) + (a[1] - d[1]).pow(2);
        edges[3] = (b[0] - c[0]).pow(2) + (b[1] - c[1]).pow(2);
        edges[4] = (b[0] - d[0]).pow(2) + (b[1] - d[1]).pow(2);
        edges[5] = (c[0] - d[0]).pow(2) + (c[1] - d[1]).pow(2);
        edges.sort();

        for i in 1..6 {
            if i < 4 && edges[i-1] == edges[i] {
                continue
            } else if i == 4 && edges[i-1] < edges[i] {
                continue
            } else if i > 4 && edges[i-1] == edges[i] {
                writeln!(output, "1")?;
            } else {
                writeln!(output, "0")?;
                break
            }
        }
    }

    Ok(())
}