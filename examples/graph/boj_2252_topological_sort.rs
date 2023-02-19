// https://www.acmicpc.net/problem/2252
// O(N + M)
// DAG

use std::{
    collections::VecDeque,
    error::Error,
    io::{self, prelude::*, BufWriter},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut input = Scanner::new(&input);
    let (n, m) = (input.read::<usize>(), input.read::<usize>());

    let mut graph = vec![vec![]; n + 1];
    let mut indegree = vec![0; n + 1];

    (0..m).for_each(|_| {
        let (a, b) = (input.read::<usize>(), input.read::<usize>());
        graph[a].push(b);
        indegree[b] += 1;
    });

    let mut dq: VecDeque<usize> = (1..n + 1)
        .into_iter()
        .filter_map(|i| (indegree[i] == 0).then(|| i))
        .collect();

    while !dq.is_empty() {
        let num = dq.pop_front().unwrap();
        write!(output, "{} ", num)?;
        graph[num].iter().for_each(|&i| {
            indegree[i] -= 1;
            if indegree[i] == 0 {
                dq.push_back(i)
            }
        });
    }

    Ok(())
}
