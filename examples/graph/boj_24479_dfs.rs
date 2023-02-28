// https://www.acmicpc.net/problem/24479

// dfs특) 스택을 사용해 방문할 노드를 저장하고,
// 방문하지 않은 adj_neighbor가 남아있지 않을 때까지 노드를 재귀적으로 방문함
// 처음 방향을 정하면 가능한 깊이 방문함(재귀). 그런 다음 역추적하여 다른 분기를 방문함.

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

static mut CNT: usize = 1;

fn main() -> Result<(), Box<dyn Error>> {
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

    (1..n+1).for_each(|i| graph[i].sort());

    let mut visited = vec![0; n+1];

    unsafe { dfs(&mut visited, &graph, r); }

    for i in 1..=n {
        writeln!(output, "{}", visited[i])?;
    }

    Ok(())
}

unsafe fn dfs(visited: &mut Vec<usize>, adj_list: &Vec<Vec<usize>>, start_node: usize) {
    visited[start_node] = CNT;
    for &x in adj_list[start_node].iter() {
        if visited[x] == 0 {
            CNT += 1;
            dfs(visited, adj_list, x);
        }
    }
}