// https://www.acmicpc.net/problem/10026

use std::{
    io::{self, prelude::*},
    collections::VecDeque,
};

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let mut read = || lines.next().unwrap();
    let n = read().parse::<usize>().unwrap();

    let mut graph = Vec::with_capacity(n);

    for _ in 0..n {
        graph.push(read().as_bytes().to_owned());
    }

    let mut cont_graph = graph.clone();

    for i in 0..n {
        for j in 0..n {
            if graph[i][j] == b'G' {
                cont_graph[i][j] = b'R'
            }
        }
    }

    let mut cnt = 0;
    let mut cnt2 = 0;
    for i in 0..n {
        for j in 0..n {
            if graph[i][j] != 0 {
                bfs(i, j, graph[i][j] as usize, &mut graph, n);
                cnt += 1
            }

            if cont_graph[i][j] != 0 {
                bfs(i, j, cont_graph[i][j] as usize, &mut cont_graph, n);
                cnt2 += 1
            }
        }
    }

    println!("{} {}", cnt, cnt2);
}

fn bfs(x: usize, y: usize, color: usize, graph: &mut Vec<Vec<u8>>, n: usize) {
    let mut q = VecDeque::from([(x, y)]);
    graph[x][y] = 0;

    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();

        for i in 0..4 {
            let (nx, ny) = (x as i32 + DX[i], y as i32 + DY[i]);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                continue
            }
            if graph[nx as usize][ny as usize] as usize == color {
                graph[nx as usize][ny as usize] = 0;
                q.push_back((nx as usize, ny as usize));
            }
        }
    }
}
