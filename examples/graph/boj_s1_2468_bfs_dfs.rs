// https://www.acmicpc.net/problem/2468

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

const DIR: [(i16, i16); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

macro_rules! read_input {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            $input.trim().parse::<$type>()
        }
    }
}

macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $n:expr, $type:ty) => {
        {
            (0..$n).map(|_| {
                $input.clear();
                $reader.read_line(&mut $input).expect("Failed to read");
                $input.split_ascii_whitespace().map(|s| s.parse::<$type>().expect("Failed to parse")).collect::<Vec<$type>>()
            }).collect::<Vec<Vec<$type>>>()
        }
    }
}

fn _bfs(graph: &mut Vec<Vec<bool>>, x: i16, y: i16) {
    graph[x as usize][y as usize] = false;
    let x_len = graph.len() as i16;
    let y_len = graph[0].len() as i16;
    let mut dq = std::collections::VecDeque::from([(x, y)]);
    while let Some((a, b)) = dq.pop_front() {
        for dir in DIR {
            let nx = a + dir.0;
            let ny = b + dir.1;
            if nx >= 0 && nx < x_len && ny >= 0 && ny < y_len && graph[nx as usize][ny as usize] {
                graph[nx as usize][ny as usize] = false;
                dq.push_back((nx, ny));
            }
        }
    }
}

fn _dfs(graph: &mut Vec<Vec<bool>>, x: i16, y: i16) {
    graph[x as usize][y as usize] = false;
    let x_len = graph.len() as i16;
    let y_len = graph[0].len() as i16;

    for dir in DIR {
        let nx = x + dir.0;
        let ny = y + dir.1;
        if nx >= 0 && nx < x_len && ny >= 0 && ny < y_len && graph[nx as usize][ny as usize] {
            _dfs(graph, nx, ny);
        }
    }
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let Ok(n) = read_input!(read_buf, buf_to_string, usize) {
        let region = read_to_vec!(read_buf, buf_to_string, n, i16);
        let x = region.iter().flatten().max().unwrap();

        let mut max = 0i16;

        for i in 0..*x {
            let mut map = region.iter().map(|street|
                street.iter().map(|&house| if house > i as i16 { true } else { false } ).collect::<Vec<bool>>()
            ).collect::<Vec<Vec<bool>>>();

            let mut cnt = 0;

            for i in 0..n {
                for j in 0..n {
                    if map[i][j] {
                        _bfs(&mut map, i as i16, j as i16);
                        // _dfs(&mut map, i as i16, j as i16);
                        cnt += 1;
                    }
                }
            }

            max = max.max(cnt);
        }

        writeln!(write_buf, "{}", max)?;
    }

    Ok(())
}