// https://www.acmicpc.net/problem/24481

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, m, r) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());

    let mut graph = vec![vec![]; n+1];
    let mut visited = vec![false; n+1];
    let mut nodes_depth = vec![0; n+1];

    for _ in 0..m {
        let (u, v) = (sc.read::<usize>(), sc.read::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..n+1 {
        graph[i].sort();
        graph[i].reverse();
    }

    let mut v = Vec::from(vec![(r, 0)]);

    while !v.is_empty() {
        let (cur_node, depth) = v.pop().unwrap();
        if visited[cur_node] {
            continue
        }
        visited[cur_node] = true;
        nodes_depth[cur_node] = depth;

        for next_node in &graph[cur_node] {
            if !visited[*next_node] {
                v.push((*next_node, depth+1))
            }
        }
    }

    for i in 1..=n {
        writeln!(output, "{}", match i {
            i if i == r => 0,
            i if nodes_depth[i] == 0 => -1,
            _ => nodes_depth[i],
        })?
    }

    Ok(())
}