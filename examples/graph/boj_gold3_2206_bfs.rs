// https://www.acmicpc.net/problem/2206

use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut line = || input.next();
    let first_line = line().unwrap().unwrap();
    let (n, m) = first_line.split_once(' ').unwrap();
    let (n, m) = (n.parse::<usize>().unwrap(), m.parse::<usize>().unwrap());

    // lazy input으로 필요할때만(n만큼) 읽고 필요한 만큼만 메모리에 저장. 또한 모든 입력이 읽힐 때까지 기다리지 않고
    // lazy evaluation을 사용하여 필요한만큼 즉시 읽고 수신되는 대로(라인마다) 즉시 출력(처리)할 수 있어,
    // 한 번에 모든 입력 줄을 읽고 저장하는 방식보다 빠르다.
    // line에 input을 입력했을때, 간혹 모든 라인이 입력되기 전에 이전에 읽힌 라인이 출력되는 사례가 이런 경우이다.
    let mut grid = input.take(n).map(|line| line.unwrap().as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut visited = vec![vec![[0; 2]; m]; n];

    writeln!(output, "{}", bfs(&mut grid, &mut visited, n, m))?;
    Ok(())
}

fn bfs(grid: &mut Vec<Vec<u8>>, visited: &mut Vec<Vec<[i32; 2]>>, n: usize, m: usize) -> i32 {
    let mut dq = VecDeque::new();
    dq.push_back((0, 0, 0));
    visited[0][0][0] = 1;

    while let Some((x, y, wall)) = dq.pop_front() {
        if x == n - 1 && y == m - 1 {
            return visited[x][y][wall] as i32;
        }

        for (dx, dy) in DIR {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] == b'0' && visited[nx][ny][wall] == 0 {
                    visited[nx][ny][wall] = visited[x][y][wall] + 1;
                    dq.push_back((nx, ny, wall));
                } else if grid[nx][ny] == b'1' && wall == 0 {
                    visited[nx][ny][wall + 1] = visited[x][y][wall] + 1;
                    dq.push_back((nx, ny, wall + 1));
                }
            }
        }
    }

    return -1;
}
