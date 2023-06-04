// https://www.acmicpc.net/problem/1240
// bfs

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
    let (n, m) = (sc.read::<usize>(), sc.read::<usize>());

    let mut graph = vec![vec![]; n+1];

    for _ in 0..n-1 {
        let (a, b, d) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());
        graph[a].push((b, d));
        graph[b].push((a, d));
    }

    for _ in 0..m {
        let mut visited = vec![false; n+1];
        let (s, e) = (sc.read::<usize>(), sc.read::<usize>());

        let mut q = Vec::from(vec![(s, 0)]);
        let mut result = 0;

        while !q.is_empty() {
            let (cur_node, depth) = q.pop().unwrap();
            if visited[cur_node] {
                continue
            }
            if cur_node == e {
                result = depth;
                break
            }

            visited[cur_node] = true;

            for next_node in &graph[cur_node] {
                if !visited[next_node.0] {
                    q.push((next_node.0, depth+next_node.1))
                }
            }
            result = q.get(0).unwrap().1;
        }
        writeln!(output, "{}", result)?;
    }
    Ok(())
}