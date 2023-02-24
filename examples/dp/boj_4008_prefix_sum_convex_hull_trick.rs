// https://www.acmicpc.net/problem/4008
// O(N.pow(2)) -> if a[i] <= a[i+1] { O(N) } else { O(N lg N) }
// https://justicehui.github.io/hard-algorithm/2019/01/25/CHT/

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
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

// CHT를 구하기 위한 line
struct Line {
    slope: i64,
    intercept: i64,
}

impl Line {
    fn evaluate_at(&self, x: i64) -> i64 {
        self.slope * x + self.intercept
    }

    fn find_intersection_x(&self, line: &Line) -> f64 {
        (self.intercept - line.intercept) as f64 / (line.slope - self.slope) as f64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (n, a, b, c) = (sc.read::<usize>(), sc.read::<i64>(), sc.read::<i64>(), sc.read::<i64>());

    let mut v = vec![0; n + 1];
    let mut prefix_sum = vec![0; n + 1];
    for i in 1..=n {
        v[i] = sc.read::<i64>();
        prefix_sum[i] = v[i] + prefix_sum[i - 1];
    }

    let mut dq = VecDeque::new();
    dq.push_front(Line { slope: 0, intercept: 0 });
    let mut ans = 0;
    for i in 1..=n {
        while dq.len() >= 2 && dq[0].evaluate_at(prefix_sum[i]) <= dq[1].evaluate_at(prefix_sum[i]) {
            dq.pop_front();
        }

        ans = dq.front().unwrap().evaluate_at(prefix_sum[i]);

        let cur = Line {
            slope: -2 * a * prefix_sum[i],
            intercept: a * prefix_sum[i].pow(2) - b * prefix_sum[i] + ans + a * prefix_sum[i].pow(2) + b * prefix_sum[i] + c,
        };
        while dq.len() >= 2 && cur.find_intersection_x(dq.back().unwrap()) <= dq.back().unwrap().find_intersection_x(&dq[dq.len() - 2]) {
            dq.pop_back();
        }
        dq.push_back(cur);
    }

    writeln!(output, "{}", ans + a * prefix_sum[n].pow(2) + b * prefix_sum[n] + c)?;

    Ok(())
}