// https://www.acmicpc.net/problem/2166
// shoelace formula
// https://namu.wiki/w/%EC%8B%A0%EB%B0%9C%EB%81%88%20%EA%B3%B5%EC%8B%9D
// https://ko.wikipedia.org/wiki/%EC%8B%A0%EB%B0%9C%EB%81%88_%EA%B3%B5%EC%8B%9D

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut line = || input.next().unwrap().unwrap();
    let n = line().parse::<usize>().unwrap();

    let mut sum = 0;

    let mut xs = (0, 0);
    let mut ys = (0, 0);

    let (mut x0, mut y0) = (0, 0);

    for i in 0..n {
        let l = line();
        let (x, y) = l.split_once(' ').unwrap();
        let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        if i == 0 {
            (x0, y0) = (x, y);
            xs.1 = x;
            ys.1 = y;
        }
        xs = (xs.1, x);
        ys = (ys.1, y);

        sum += xs.0 * ys.1 - xs.1 * ys.0;
    }

    // 첫번째 점과 마지막 점이 동일하지 않을 경우를 대비
    if x0 != xs.1 || y0 != ys.1 {
        sum += xs.1 * y0 - x0 * ys.1;
    }

    writeln!(output, "{:.1}", (sum as f64 / 2.0).abs())?;

    Ok(())
}