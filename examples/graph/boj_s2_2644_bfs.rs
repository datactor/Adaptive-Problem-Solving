// https://www.acmicpc.net/problem/2644

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_to_num {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            let n = $input.trim().parse::<$type>();
            $input.clear();
            $reader.read_line(&mut $input)?;
            let (a, b) = $input.trim().split_once(' ').expect("no a & b");
            let (a, b) = (a.parse::<$type>(), b.parse::<$type>());
            $input.clear();
            $reader.read_line(&mut $input)?;
            let m = $input.trim().parse::<$type>();
            (n, a, b, m)
        }
    }
}

macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $n:expr, $m:expr, $type:ty) => {
        {
            let mut vec = vec![vec![]; $n+1];
            for _ in 0..$m {
                $input.clear();
                $reader.read_line(&mut $input)?;
                let (x, y) = $input.trim().split_once(' ').expect("no x & y");
                let (x, y) = (x.parse::<$type>().expect("no 1st iter"), y.parse::<$type>().expect("no 2nd iter"));
                vec[x as usize].push(y);
                vec[y as usize].push(x);
            }
            vec
        }
    }
}

fn bfs(graph: &mut Vec<Vec<i8>>, x: i8, y: usize) -> usize {
    let x_len = graph.len();
    let mut degree_with_x = vec![0; x_len];
    let mut dq = std::collections::VecDeque::from([x]);
    while let Some(a) = dq.pop_front() {
        for i in &graph[a as usize] {
            if degree_with_x[*i as usize] == 0 {
                degree_with_x[*i as usize] = degree_with_x[a as usize] + 1;
                dq.push_back(*i as i8);
            }
        }
    }
    degree_with_x[y]
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let (Ok(n), Ok(x), Ok(y), Ok(m)) = read_to_num!(read_buf, buf_to_string, usize) {
        let mut relations = read_to_vec!(read_buf, buf_to_string, n, m, i8);
        let degree_with_x = bfs(&mut relations, x as i8, y);
        writeln!(write_buf, "{:?}", if degree_with_x > 0 { degree_with_x as i8 } else { -1 })?
    }

    Ok(())
}