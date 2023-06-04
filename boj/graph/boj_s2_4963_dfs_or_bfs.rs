// https://www.acmicpc.net/problem/4963

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

const DX: [i8; 8] = [1, -1, 0, 0, 1, -1, 1, -1];
const DY: [i8; 8] = [0, 0, -1, 1, -1, 1, 1, -1];

macro_rules! read_line_to_nums {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let (w, h) = $input.trim().split_once(' ').expect("Failed to read (w, h)");
            (w.parse::<$type>(), h.parse::<$type>())
        }
    }
}

macro_rules! read_lines_to_vec {
    ($reader:expr, $input:expr, $num:expr, $type:ty) => {
        {
            let iter = (0..$num).map(|_| {
                $input.clear();
                $reader.read_line(&mut $input).expect("Failed to read");
                $input.split_ascii_whitespace()
                    .map(|s| {
                    if s == "1" { true } else { false }
                })
                    .collect()
            }).collect::<Vec<Vec<$type>>>();
            iter
        }
    }
}

fn _bfs(graph: &mut Vec<Vec<bool>>, x: i8, y: i8) {
    graph[x as usize][y as usize] = false;
    let x_len = graph.len() as i8;
    let y_len = graph[0].len() as i8;
    let mut dq = std::collections::VecDeque::from([(x, y)]);
    while let Some((a, b)) = dq.pop_front() {
        for i in 0..8 {
            let nx = a + DX[i];
            let ny = b + DY[i];
            if nx >= 0 && nx < x_len && ny >= 0 && ny < y_len && graph[nx as usize][ny as usize] {
                graph[nx as usize][ny as usize] = false;
                dq.push_back((nx, ny));
            }
        }
    }
}

fn _dfs(graph: &mut Vec<Vec<bool>>, x: i8, y: i8) {
    graph[x as usize][y as usize] = false;
    let x_len = graph.len() as i8;
    let y_len = graph[0].len() as i8;

    for i in 0..8 {
        let nx = x + DX[i];
        let ny = y + DY[i];
        if nx >= 0 && nx < x_len && ny >= 0 && ny < y_len && graph[nx as usize][ny as usize] {
            _dfs(graph, nx, ny);
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buffer = BufReader::new(io::stdin().lock());
    let mut write_buffer = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    while let (Ok(w), Ok(h)) = read_line_to_nums!(read_buffer, buf_to_string, usize) {
        if (w, h) == (0, 0) { break }

        let mut cnt = 0;

        let mut map = read_lines_to_vec!(read_buffer, buf_to_string, h, bool);
        for i in 0..h {
            for j in 0..w {
                if map[i][j] {
                    // _bfs(&mut map, i as i8, j as i8);
                    _dfs(&mut map, i as i8, j as i8);
                    cnt += 1;
                }
            }
        }
        writeln!(write_buffer, "{}", cnt)?;
    }

    Ok(())
}