// https://www.acmicpc.net/problem/1167
// 트리의 지름? 임의의 두 점 사이의 거리 중 가장 긴 것.
//
// 트리의 지름 구하는 방법(https://jioneprogstdy.tistory.com/77, https://bedamino.tistory.com/15)
// 임의의 점(A)에서 가장 먼 지점 B를 찾는다
// B에서 가장 먼 지점(C)를 찾는다
// B~C의 거리가 트리의 지름
//
// dfs를 돌려도 트리 지름 공식을 알지 못하면 풀기 어렵다.

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace();
    let mut read = || input.next().unwrap().parse::<usize>(); // 클로저 함수를 변수로 두면 변수 호출에도 함수를 실행 시킬 수 있음.

    let v: usize = read().unwrap();
    let mut graph = vec![vec![]; v + 1];
    graph.push(vec![(0, 0)]);

    for _ in 0..v {
        let i = read().unwrap();
        loop {
            let x = if let Ok(t) = read() { t } else { break };
            graph[i].push((x, read().unwrap()));
        }
    }

    // 탐색 여부 기록
    let mut visited = vec![-1; v + 1];
    visited[1] = 0;
    dfs(1, 0, &graph, &mut visited);

    let max = visited.iter().max().unwrap(); // 1번 노드에서 가장 먼 노드와의 거리
    let start = visited.iter().position(|x| x == max).unwrap(); // 1번 노드에서 가장 먼 노드 idx

    visited = vec![-1; v + 1]; // 탐색 여부 vec 초기화
    visited[start] = 0; // 1번 노드에서 가장 먼 node부터 다시 탐색
    dfs(start, 0, &graph, &mut visited);

    println!("{}", visited.iter().max().unwrap());
    Ok(())
}

fn dfs(idx: usize, v: usize, graph: &Vec<Vec<(usize, usize)>>, visited: &mut Vec<i32>) {
    // idx번째 노드와 연결된 모든 (노드, 거리) 탐색
    for (node, dist) in &graph[idx] {
        if visited[*node] == -1 {
            // 아직 탐색하지 않은 노드라면
            visited[*node] = (dist + v) as i32; // 탐색여부 vec(visited)에 노드간 거리 + 탐색까지 걸린 거리(과거)로 총 탐색거리를 구해줌
            dfs(*node, dist + v, graph, visited); // 새로운 스택프레임 생성해서 재귀적 탐색
        }
    }
}
