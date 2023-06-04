// https://www.acmicpc.net/problem/24444
// bfs의 시간복잡도는 m(에지 수)이지만, 위 문제의 경우 정렬연산이 오버헤드가 더 크다.
// O(nlgn + m)

// BFS is a memory-intensive algorithm that creates a queue to visit nodes in an unweighted graph.
// 노드를 방문할 때마다 방문하지 않은 adj_neighbor를 추후 처리를 위한 대기열에 추가함.
// 대기열을 만들기 때문에 메모리를 많이 사용하지만, 비가중 그래프에서 시작 노드에서 다른 노드까지의
// 최단 경로(edge 수 측면에서)를 찾는 것이 보장됨.
//
// bfs는 거리가 증가하는 순서대로 노드를 방문하기 때문에(즉 해당 노드에 최초 방문한 시점이 최단 거리)
// 노드에 대한 최단 경로는 항상 순회에서 먼저 발견되고 방문된다. 따라서 최단 경로 찾기에 최적화 되어 있다.

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