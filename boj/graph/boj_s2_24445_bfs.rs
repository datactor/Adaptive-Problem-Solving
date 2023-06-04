// https://www.acmicpc.net/problem/24445

use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);
    let (n, m, r) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());

    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (sc.read::<usize>(), sc.read::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..=n {
        graph[i].sort();
        graph[i].reverse();
    }
    let mut visited = vec![0; n+1];

    bfs(&graph, &mut visited, r);

    for i in 1..n+1 {
        writeln!(output, "{}", visited[i])?;
    }

    Ok(())
}

fn bfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<usize>, start: usize) {
    let mut dq = VecDeque::from([start]);
    visited[start] = 1;
    let mut tmp = 1;
    while let Some(inq) = dq.pop_front() {
        for v in &graph[inq] {
            if visited[*v] == 0 {
                tmp += 1;
                visited[*v] = tmp;
                dq.push_back(*v)
            }
        }
    }
}