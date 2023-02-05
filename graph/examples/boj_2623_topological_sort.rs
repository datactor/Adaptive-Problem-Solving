// https://www.acmicpc.net/problem/2623
// DAG(topological sort)
// O(n + m)

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
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
    let mut graph = vec![vec![]; n+1]; // 노드 seq 정렬. atomic은 아님
    let mut indegree = vec![0; n+1]; // 위상 정렬

    for _ in 0..m {
        let num = scanner.read::<usize>();
        let sub: Vec<usize> = (0..num).map(|_| scanner.read::<usize>()).collect();
        for j in 0..num-1 {
            graph[sub[j]].push(sub[j+1]); // 선행 노드별로 후행 노드를 push
            indegree[sub[j+1]] += 1; // 차수 기입
        }
    }


    let mut dq = VecDeque::new();
    for i in 1..n+1 {
        if indegree[i] == 0 {
            dq.push_back(i) // dq에 차수가 0인 노드를 우선 기입한다.
        }
    }

    let mut result = vec![];
    while !dq.is_empty() {
        let num = dq.pop_front().unwrap(); // 선입 선출
        result.push(num);
        for i in &graph[num] { // 선출된 노드를 그래프서 찾고 후행 노드를 모두 불러옴
            indegree[*i] -= 1; // 불러온 후행 노드의 차수를 줄인다
            if indegree[*i] == 0 { // 차수를 줄였을 때 0일 경우 dq에 넣는다.
                dq.push_back(*i)
            }
        }
    }

    // 모든 노드가 불러질 수 없다면 보조 pd들의 seq들이 상충된다는 것으로
    // (result의 elements수가 node수보다 적다면 DAG가 아니라는 뜻으로(단방향 비순환그래프가 아니라 cycle이 있다는 뜻))
    // error처리한다.
    if result.len() != n {
        writeln!(output, "0")?;
    } else {
        for i in result {
            writeln!(output, "{}", i)?;
        }
    }

    Ok(())
}