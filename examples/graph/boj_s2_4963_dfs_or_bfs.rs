// https://www.acmicpc.net/problem/4963

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

const DX: [i32; 8] = [1, -1, 0, 0, 1, -1, 1, -1];
const DY: [i32; 8] = [0, 0, -1, 1, -1, 1, 1, -1];

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
                    .map(|s| s.parse::<$type>().expect("Failed to parse"))
                    .collect()
            }).collect::<Vec<Vec<$type>>>();
            iter
        }
    }
}

fn bfs(graph: &mut Vec<Vec<i8>>, x: usize, y: usize) {
    graph[x][y] = 0;
    let x_len = graph.len();
    let y_len = graph[0].len();
    let mut dq = std::collections::VecDeque::new();
    dq.push_back((x, y));
    while let Some((a, b)) = dq.pop_front() {
        for i in 0..8 {
            let nx = a as i32 + DX[i];
            let ny = b as i32 + DY[i];
            if nx >= 0 && nx < x_len as i32 && ny >= 0 && ny < y_len as i32 && graph[nx as usize][ny as usize] == 1 {
                graph[nx as usize][ny as usize] = 0;
                dq.push_back((nx as usize, ny as usize));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();

    while let (Ok(w), Ok(h)) = read_line_to_nums!(reader, buffer, usize) {
        if (w, h) == (0, 0) { break }

        let mut cnt = 0;

        let mut map = read_lines_to_vec!(reader, buffer, h, i8);
        for i in 0..h {
            for j in 0..w {
                if map[i][j] == 1 {
                    bfs(&mut map, i, j);
                    cnt += 1;
                }
            }
        }
        writeln!(writer, "{}", cnt)?;
    }

    Ok(())
}