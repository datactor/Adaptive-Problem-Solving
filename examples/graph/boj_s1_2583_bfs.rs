// https://www.acmicpc.net/problem/2583

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

const DIR: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

macro_rules! read_to_nums {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            let mut iter = $input.split_ascii_whitespace();
            let m = iter.next().expect("no 1st iter").parse::<$type>();
            let n = iter.next().expect("no 2nd iter").parse::<$type>();
            let k = iter.next().expect("no 3rd iter").parse::<$type>();
            (m, n, k)
        }
    }
}

macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $m:expr, $n:expr, $k:expr, $type:ty) => {
        {
            let mut map = vec![vec![true; $n]; $m];
            for _ in 0..$k {
                $input.clear();
                $reader.read_line(&mut $input)?;
                let mut iter = $input.split_ascii_whitespace();
                let x1 = iter.next().expect("no 1st iter").parse::<$type>().expect("Failed to parse");
                let y1 = iter.next().expect("no 2nd iter").parse::<$type>().expect("Failed to parse");
                let x2 = iter.next().expect("no 3rd iter").parse::<$type>().expect("Failed to parse");
                let y2 = iter.next().expect("no 4th iter").parse::<$type>().expect("Failed to parse");
                let a = ($m - y1, x1);
                let b = ($m - y2, x2);
                for i in b.0..a.0 {
                    for j in a.1..b.1 {
                        map[i][j] = false
                    }
                }
            }
            map
        }
    }
}

fn bfs(graph: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    graph[x][y] = false;
    let x_len = graph.len();
    let y_len = graph[0].len();
    let mut dq = std::collections::VecDeque::from([(x, y)]);
    let mut cnt = 1;
    while let Some((a, b)) = dq.pop_front() {
        for dir in DIR {
            let nx = a as i8 + dir.0;
            let ny = b as i8 + dir.1;
            if nx >= 0 && nx < x_len as i8 && ny >= 0 && ny < y_len as i8 && graph[nx as usize][ny as usize] {
                graph[nx as usize][ny as usize] = false;
                dq.push_back((nx as usize, ny as usize));
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let (Ok(m), Ok(n), Ok(k)) = read_to_nums!(read_buf, buf_to_string, usize) {
        let mut map = read_to_vec!(read_buf, buf_to_string, m, n, k, usize);

        let mut cnt = 0;
        let mut areas = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if map[i][j] {
                    areas.push(bfs(&mut map, i, j));
                    cnt += 1;
                }
            }
        }
        areas.sort();
        writeln!(write_buf, "{}", cnt)?;
        for area in areas {
            write!(write_buf, "{} ", area)?
        }
    }
    Ok(())
}