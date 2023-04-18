// https://www.acmicpc.net/problem/1012
// O(t * m * n)

use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

const DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let t = sc.read::<usize>();
    for _ in 0..t {
        let (m, n, k) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());
        let mut grid = vec![vec![0; m]; n];
        for _ in 0..k {
            let (x, y) = (sc.read::<usize>(), sc.read::<usize>());
            grid[y][x] = 1;
        }
        writeln!(output, "{}", count_regions(&mut grid, m, n))?;
    }

    Ok(())
}

fn count_regions(grid: &mut Vec<Vec<usize>>, m: usize, n: usize) -> usize {
    let mut cnt = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                bfs(grid, i, j);
                cnt += 1;
            }
        }
    }

    cnt
}

fn bfs(grid: &mut Vec<Vec<usize>>, i: usize, j: usize) {
    let mut dq = VecDeque::new();
    dq.push_back((i, j));

    while let Some((x, y)) = dq.pop_front() {
        if grid[x][y] == 0 {
            continue;
        }

        grid[x][y] = 0;

        for i in DIR {
            let (nx, ny) = (x as i32 + i.0, y as i32 + i.1);
            if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < grid[0].len() as i32 && grid[nx as usize][ny as usize] == 1 {
                dq.push_back((nx as usize, ny as usize))
            }
        }
    }
}
