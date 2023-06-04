// https://www.acmicpc.net/problem/7569
// O(m * n * h * 6)

use std::{
    io::{self, prelude::*, BufWriter},
    collections::VecDeque,
};

const DIR: [(i32, i32, i32); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

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

    let (m, n, h) = (sc.read::<usize>(), sc.read::<usize>(), sc.read::<usize>());

    let mut starting_point = VecDeque::new();
    let mut grid = vec![vec![vec![0; m]; n]; h];

    for z in 0..h {
        for y in 0..n {
            for x in 0..m {
                let tomato_state = sc.read::<i32>();
                grid[z][y][x] = tomato_state;
                if tomato_state == 1 {
                    starting_point.push_back((z, y, x));
                }
            }
        }
    }

    writeln!(output, "{}", elapsed_days(&mut grid, &mut starting_point, h, m, n))?;

    Ok(())
}

fn elapsed_days(grid: &mut Vec<Vec<Vec<i32>>>, starting_point: &mut VecDeque<(usize, usize, usize)>, h: usize, m: usize, n: usize) -> i32 {
    let mut days = 0;

    while !starting_point.is_empty() {
        let mut next_points = VecDeque::new();

        while let Some((z, y, x)) = starting_point.pop_front() {
            for (dz, dy, dx) in DIR {
                let (nz, ny, nx) = (z as i32 + dz, y as i32 + dy, x as i32 + dx);

                if nz >= 0 && nz < h as i32 && ny >= 0 && ny < n as i32 && nx >= 0 && nx < m as i32 && grid[nz as usize][ny as usize][nx as usize] == 0 {
                    grid[nz as usize][ny as usize][nx as usize] = 1;
                    next_points.push_back((nz as usize, ny as usize, nx as usize));
                }
            }
        }

        if !next_points.is_empty() {
            days += 1;
        }

        *starting_point = next_points;

    }

    for z in 0..h {
        for y in 0..n {
            for x in 0..m {
                if grid[z][y][x] == 0 {
                    return -1;
                }
            }
        }
    }

    days
}