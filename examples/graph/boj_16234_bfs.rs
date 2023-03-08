// https://www.acmicpc.net/problem/16234

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

fn bfs(grid: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<i32>>, i: usize, j: usize, lower_bound: i32, upper_bound: i32) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    queue.push_back((i, j));
    let mut tmp = Vec::new();
    tmp.push((i, j));
    while let Some((x, y)) = queue.pop_front() {
        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let nx = x as i32 + d.0;
            let ny = y as i32 + d.1;
            if nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid.len() as i32 && visited[nx as usize][ny as usize] == 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if (grid[x][y] - grid[nx][ny]).abs() >= lower_bound && (grid[x][y] - grid[nx][ny]).abs() <= upper_bound {
                    visited[nx][ny] = 1;
                    queue.push_back((nx, ny));
                    tmp.push((nx, ny));
                }
            }
        }
    }
    tmp
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(io::stdout().lock());

    let dimensions: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut grid = vec![vec![0; dimensions[0] as usize]; dimensions[0] as usize];
    for i in 0..dimensions[0] as usize {
        let row: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid[i] = row;
    }

    let mut cnt = 0;
    loop {
        let mut visited = vec![vec![0; dimensions[0] as usize]; dimensions[0] as usize];
        let mut is_valid = false;
        for i in 0..dimensions[0] as usize {
            for j in 0..dimensions[0] as usize {
                if visited[i][j] == 0 {
                    visited[i][j] = 1;
                    let tmp = bfs(&mut grid, &mut visited, i, j, dimensions[1], dimensions[2]);
                    if tmp.len() > 1 {
                        is_valid = true;
                        let avg = tmp.iter().map(|&(x, y)| grid[x][y]).sum::<i32>() / tmp.len() as i32;
                        for &(x, y) in &tmp {
                            grid[x][y] = avg;
                        }
                    }
                }
            }
        }
        if !is_valid {
            break;
        }
        cnt += 1;
    }
    writeln!(output, "{}", cnt)?;

    Ok(())
}