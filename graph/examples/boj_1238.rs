// https://www.acmicpc.net/problem/1238
// dfs를 사용한 풀이
//     for i in 1..n+1 {
//         let min_1 = dfs(i, x); i to x 최소값
//         let min_2 = dfs(x, i); x to i 최소값
//         max = i32::max(max, min_1 + min_2);
//     }
//
// dijkstra
// 각 노드에서 노드로 가는 최소길이를 벡터안에 넣어둔다.
// i to x, x to i 각각의 최소 길이의 합의 최대값을 구함

use std::{
    io::{self, prelude::*},
    error::Error,
    collections::BinaryHeap,
};

struct Graph {
    adj_list: Vec<Vec<(i32, i32)>>,
    distances: Vec<i32>,
}

impl Graph {
    fn new(adj: Vec<Vec<(i32, i32)>>, v: i32) -> Self {
        let d_min = vec![i32::MAX; (v + 1) as usize];
        Self {
            adj_list: adj,
            distances: d_min,
        }
    }

    fn find_shortest_path(&mut self, start: i32) {
        let mut q = BinaryHeap::new();
        q.push((0, start));
        self.distances[start as usize] = 0;

        while !q.is_empty() {
            let (mut d_min, node) = q.pop().unwrap();
            d_min *= -1;
            if self.distances[node as usize] < d_min {
                continue;
            }

            for (neighbor, weight) in self.adj_list[node as usize].iter() {
                let cost = *weight + d_min;
                if cost < self.distances[*neighbor as usize] {
                    self.distances[*neighbor as usize] = cost;
                    q.push((cost * -1, *neighbor));
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut read = || input.next().unwrap();

    let (n, m, x) = (read() as usize, read(), read());

    let mut adj_list1 = vec![vec![]; n + 1];
    let mut adj_list2 = vec![vec![]; n + 1];

    for (s, e, cost) in (0..m).map(|_| (read(), read(), read())) {
        adj_list1[s as usize].push((e, cost));
        adj_list2[e as usize].push((s, cost));
    }

    let mut g1 = Graph::new(adj_list1, n as i32);
    let mut g2 = Graph::new(adj_list2, n as i32);
    g1.find_shortest_path(x);
    g2.find_shortest_path(x);
    let mut max = 0;
    for i in 1..=n {
        max = Ord::max(max, g1.distances[i] + g2.distances[i]);
    }
    println!("{}", max);

    Ok(())
}