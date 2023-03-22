// https://www.acmicpc.net/problem/24444

use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn read<T:std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, m, r) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());
    let mut graph = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (sc.read::<usize>(), sc.read::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![0; n+1];

    for i in 0..n {
        graph[i].sort();
    }

    bfs(&graph, &mut visited, r);

    for i in 1..=n {
        writeln!(output, "{}", visited[i])?;
    }

    Ok(())
}

fn bfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<usize>, start: usize) {
    let mut deque = VecDeque::from([&graph[start]]);
    visited[start] = 1;
    let mut tmp = 1;
    while let Some(inq) = deque.pop_front() {
        for v in inq {
            if visited[*v] == 0 {
                visited[*v] = tmp+1;
                tmp = visited[*v];
                deque.push_back(&graph[*v])
            }
        }
    }
}