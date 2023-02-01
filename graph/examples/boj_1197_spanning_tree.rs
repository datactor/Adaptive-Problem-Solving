// https://www.acmicpc.net/problem/1197
// https://gmlwjd9405.github.io/2018/08/28/algorithm-mst.html
// https://www.crocus.co.kr/733
// 기본적으로 '트리'는 사이클이 존재하지 않는 그래프.

// 크루스칼 알고리즘 vs 프림 알고리즘
//
// 크루스칼 알고리즘 시간 복잡도 :: O(E * lg E)
// 프림 알고리즘 시간 복잡도 :: O(E * lg V)
// 간선의 개수가 많으면 프림, 적으면 크루스칼을 사용하는 것이 유리할 듯 함.

use std::{
    io::{self, prelude::*},
    error::Error,
    collections::BinaryHeap,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut read = || input.next().unwrap();

    let (v, e) = (read(), read());

    let mut graph = vec![vec![]; (v + 1) as usize];
    for _ in (0..e) {
        let (u, v, w) = (read(), read(), read());
        graph[u as usize].push((v, w));
        graph[v as usize].push((u, w));
    };

    println!("{}", prim(1, 0, graph, v as usize));

    Ok(())
}

fn prim(start: usize, weight: i32, graph: Vec<Vec<(i32, i32)>>, n: usize) -> i32 {
    let mut visited = vec![0; n+1]; // node 방문 여부
    let mut q = BinaryHeap::from([(weight, start)]); // 가중치를 앞에 둬서 힙 사용
    let mut sum = 0; // 가중치의 합
    let mut cnt = 0; // 간선의 개수

    while cnt < n {
        let (k, v) = q.pop().unwrap(); // 가장 작은 수부터 pop(러스트의 heap pop은 큰수부터 빼지만 부호를 바꿔 넣었음)
        if visited[v] == 1 {
            continue // 방문한 지점은 continue
        }
        visited[v] = 1; // 방문 처리.
        sum -= k;  // 해당 정점까지의 가중치를 더해준다
        cnt += 1; // 간선의 개수를 더해줌(최종 간선의 개수는 n-1 고정)
        for (u, w) in &graph[v] { // 해당 node의 간선정보를 모두 불러와서
            q.push((w*-1, *u as usize)) // 힙에 넣는다.
        }
    } sum
}