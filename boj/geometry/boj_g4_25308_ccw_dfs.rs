// https://www.acmicpc.net/problem/25308

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_line(&mut input)?;

    let stat = input.split_ascii_whitespace().map(|s| s.parse::<f64>().unwrap()).collect::<Vec<f64>>();

    let mut new_stat = [0f64; 8];
    let mut visited = [false; 8];
    let mut ans = 0;
    dfs(&stat, &mut new_stat, &mut visited, 0, &mut ans);
    writeln!(output, "{}", ans)?;

    Ok(())
}

fn ccw(v: &mut [f64; 8]) -> bool {
    for i in 0..8 {
        let a = i;
        let b = (i + 1) % 8;
        let c = (i + 2) % 8;
        if v[a] * v[c] * 2f64.sqrt() > v[b] * (v[a] + v[c]) {
            return false;
        }
    }
    true
}

fn dfs(stat: &Vec<f64>, new_stat: &mut [f64; 8], visited: &mut [bool; 8], cnt: usize, ans: &mut i32) {
    if cnt == 8 {
        *ans += ccw(new_stat) as i32;
        return;
    }
    for i in 0..8 {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        new_stat[cnt] = stat[i];
        dfs(stat, new_stat, visited, cnt + 1, ans);
        visited[i] = false;
    }
}