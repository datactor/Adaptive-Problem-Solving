// https://www.acmicpc.net/problem/1766
// Directed Acycleic Graph(Topological sort)
// 문제 이해가 중요하다. 차수가 0인 노드가 우선순위 1순위임. 그 중에 낮은 번호가 우선 순위 2순위.
// (예를들어 1 3, 3 6, 3 2, 5 4가 있으면 1 3 2 6 5 4 순으로 풀면 됨.)
// 우선순위를 확증하기 어려웠다.

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::BinaryHeap,
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

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());
    let mut graph = vec![Vec::new(); n+1];
    let mut indegree = vec![0; n+1];
    let mut hq = BinaryHeap::new();

    // Topological sort
    (0..m).for_each(|_| {
        let (a, b) = (scanner.read::<usize>(), scanner.read::<i32>());
        graph[a].push(b); // graph를 그리고
        indegree[b as usize] += 1; // 차수 vec 작성
    });

    for i in 1..n+1 {
        if indegree[i] == 0 { // indgree(진입 차수)가 0인 node를 hq에 삽입
            hq.push(-(i as i32));
        }
    }

    let mut result = Vec::new();

    while !hq.is_empty() {
        let tmp = -hq.pop().unwrap(); // q에서 가장 작은(우선순위 1순위) 원소를 꺼내
        result.push(tmp); // 먼저 풀고,
        for i in &graph[tmp as usize] { // graph[tmp]에 있는 모든 원소의 차수를 하나씩 낮춤
            indegree[*i as usize] -= 1;
            if indegree[*i as usize] == 0 { // 차수를 낮췄다면 다시 우선순위 1순위(차수가 0)인 node를 hq에 push
                hq.push(i * -1);
            }
        }
    }

    for i in result {
        write!(output, "{} ", i)?;
    }

    Ok(())
}