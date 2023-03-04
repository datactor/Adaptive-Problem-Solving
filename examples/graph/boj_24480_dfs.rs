// https://www.acmicpc.net/problem/24480

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    rc::Rc,
    cell::RefCell,
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

    for _ in 0..m {
        let (u, v) = (sc.read::<usize>(), sc.read::<usize>());
        graph[u].push(v);
        graph[v].push(u);
    }

    for i in 1..n+1 {
        graph[i].sort();
        graph[i].reverse();
    }

    let mut visited = vec![0; n+1];

    let cnt = Rc::new(RefCell::new(1));

    dfs(&mut visited, &graph, r, &cnt);

    for i in 1..=n {
        writeln!(output, "{}", visited[i])?;
    }

    Ok(())
}

fn dfs(visited: &mut Vec<usize>, adj_list: &Vec<Vec<usize>>, start_node: usize, cnt: &Rc<RefCell<usize>>) {
    visited[start_node] = cnt.replace_with(|&mut c| c+1);
    for &x in adj_list[start_node].iter() {
        if visited[x] == 0 {
            dfs(visited, adj_list, x, cnt);
        }
    }
}