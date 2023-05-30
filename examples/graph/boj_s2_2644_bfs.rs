// https://www.acmicpc.net/problem/2644

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

macro_rules! read_to_nums {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $reader.read_line(&mut $input)?;
            let n = $input.trim().parse::<$type>();
            $input.clear();
            $reader.read_line(&mut $input)?;
            let (x, y) = $input.trim().split_once(' ').expect("no x & y");
            let (x, y) = (x.parse::<$type>(), y.parse::<$type>());
            $input.clear();
            $reader.read_line(&mut $input)?;
            let m = $input.trim().parse::<$type>();
            (n, x, y, m)
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

fn bfs(graph: &mut Vec<Vec<i8>>, n: usize, x: i8, y: usize) -> io::Result<i8> {
    let mut degree_with_x = vec![0; n+1];
    let mut q = std::collections::VecDeque::from([x]);
    while let Some(relation) = q.pop_front() {
        for i in &graph[relation as usize] {
            if degree_with_x[*i as usize] == 0 {
                degree_with_x[*i as usize] = degree_with_x[relation as usize] + 1;
                q.push_back(*i);
            }
        }
    }
    degree_with_x.get(y)
        .cloned()
        .map(|n| if n == 0 { -1 } else { n })
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "The y is out of 0..=n"))
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let (Ok(n), Ok(x), Ok(y), Ok(m)) = read_to_nums!(read_buf, buf_to_string, usize) {
        let mut relations = read_to_vec!(read_buf, buf_to_string, n, m, i8);
        writeln!(write_buf, "{}", bfs(&mut relations, n, x as i8, y)?)?
    }

    Ok(())
}