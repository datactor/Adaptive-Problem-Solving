// https://www.acmicpc.net/problem/1520

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;
    let mut inputs = input.split_ascii_whitespace();
    let mut read = || inputs.next().unwrap().parse::<i32>().unwrap();

    let rows = read() as usize;
    let cols = read() as usize;

    let mut dp: Vec<Vec<i32>> = vec![vec![-1; cols]; rows];

    let map = (0..rows)
        .map(|_| (0..cols).map(|_| read()).collect::<Vec<i32>>())
        .collect();

    writeln!(output, "{}", count_paths(0, 0, &mut dp, &map, rows, cols))?;
    Ok(())
}

// dfs
fn count_paths(x: usize, y: usize, dp: &mut Vec<Vec<i32>>, map: &Vec<Vec<i32>>, rows: usize, columns: usize) -> i32 {
    if x == rows - 1 && y == columns - 1 {
        return 1;
    }
    if dp[x][y] != -1 {
        return dp[x][y];
    }
    dp[x][y] = 0;
    for i in 0..4 {
        let nx: i32 = (x as i32) + DX[i];
        let ny: i32 = (y as i32) + DY[i];
        if nx >= 0 && nx < (rows as i32) && ny >= 0 && ny < columns as i32 {
            if map[nx as usize][ny as usize] < map[x][y] {
                dp[x][y] += count_paths(nx as usize, ny as usize, dp, map, rows, columns);
            }
        }
    }
    dp[x][y]
}