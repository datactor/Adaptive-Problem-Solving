// https://www.acmicpc.net/problem/1647
// spanning tree를 이룰 때, 임의의 길 하나를 빼면 스패닝 트리를 이루는 두개의 집합으로 나뉜다.
// 프림 알고리즘 시간 복잡도 :: O(E * lg V)

use std::{
    io::{self, prelude::*},
    error::Error,
    collections::BinaryHeap,
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
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());
    let mut graph = vec![vec![]; n+1];

    (0..m).for_each(|_| {
        let (a, b, c) = (scanner.read::<usize>(), scanner.read::<usize>(), scanner.read::<i32>());
        graph[a].push((b, c));
        graph[b].push((a, c));
    });

    println!("{}", prim(1, 0, graph, n));

    Ok(())
}

fn prim(start: usize, weight: i32, graph: Vec<Vec<(usize, i32)>>, n: usize) -> i32 {
    let mut visited = vec![0; n+1]; // node 방문 여부
    let mut q = BinaryHeap::from([(weight, start)]); // 가중치를 앞에 둬서 힙 사용
    let mut sum = 0; // 가중치의 합
    let mut max = 0; // spanning tree를 이루는 길 중 가장 비싼 길의 유지비
    let mut cnt = 0; // 간선의 개수

    while cnt < n {
        let (w, idx) = q.pop().unwrap();
        if visited[idx] == 1 {
            continue // 방문한 지점은 continue
        }
        visited[idx] = 1; // 방문 처리.
        max = i32::max(max, -w);
        sum -= w;  // 해당 정점까지의 가중치를 더해준다
        cnt += 1; // 간선의 개수를 더해줌(최종 간선의 개수는 n-1 고정)

        // 해당 node의 간선정보를 모두 불러와서 힙에 넣는다.
        graph[idx].iter().for_each(|&(u, w)| q.push((-w, u as usize)));
    } sum - max
}