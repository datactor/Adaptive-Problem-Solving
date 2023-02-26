// https://www.acmicpc.net/problem/13263
// O(N lg N)
// https://stonejjun.tistory.com/50
// https://m.blog.naver.com/kks227/221418495037

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn cal_slope(point1: (i64, i64), point2: (i64, i64)) -> i64 {
    return (point1.1 - point2.1) / (point2.0 - point1.0);
}

fn insert_point(points: &mut Vec<(i64, i64)>, new_point: (i64, i64)) {
    points.push(new_point);
    while points.len() > 2 && cal_slope(points[points.len() - 3], points[points.len() - 2]) > cal_slope(points[points.len() - 2], points[points.len() - 1]) {
        points.remove(points.len() - 2);
    }
}

fn cal_y_coordinate(points: &Vec<(i64, i64)>, x: i64) -> i64 {
    let mut left = 0;
    let mut right = points.len() - 1;
    while left < right {
        let mid = (left + right) / 2;
        if cal_slope(points[mid], points[mid + 1]) <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return points[left].0 * x + points[left].1;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let n = sc.read::<usize>();

    let a: Vec<i64> = (0..n).map(|_|sc.read::<i64>()).collect();
    let b: Vec<i64> = (0..n).map(|_|sc.read::<i64>()).collect();
    let mut dp = vec![0; n];

    let mut points = vec![(b[0], 0)];
    for i in 1..n {
        dp[i] = cal_y_coordinate(&points, a[i]);
        insert_point(&mut points, (b[i], dp[i]));
    }
    writeln!(output, "{}", dp[n - 1])?;
    Ok(())
}