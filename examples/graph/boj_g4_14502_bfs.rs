use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::VecDeque,
};

const DIR: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

macro_rules! read_line_to_nums {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let nm = $input.trim().split_once(' ').expect("Failed to read N, M");
            (nm.0.parse::<$type>(), nm.1.parse::<$type>())
        }
    }
}

macro_rules! read_to_tables {
    ($reader:expr, $input:expr, $n:expr, $m:expr, $type:ty) => {
        {
            let mut susceptible_zone_idxs: Vec<i8> = Vec::with_capacity($n * $m);
            let mut epicenters: VecDeque<(i8, i8)> = VecDeque::new();
            let map = (0..$n).map(|i| {
                $input.clear();
                $reader.read_line(&mut $input).expect("Faile to read");
                $input.split_ascii_whitespace().enumerate().map(|(j, s)| {
                    let val = s.parse::<$type>().expect("Failed to parse");
                    if val == 0 {
                        susceptible_zone_idxs.push(i as i8 * $m as i8 + j as i8);
                    } else if val == 2 {
                        epicenters.push_back((i as i8, j as i8));
                    };
                    val
                }).collect::<Vec<$type>>()
            }).collect::<Vec<Vec<$type>>>();
            (susceptible_zone_idxs, map, epicenters)
        }
    }
}

fn _bfs(graph: &mut Vec<Vec<i8>>, dq: &mut VecDeque<(i8, i8)>) -> i8 {
    let mut cnt = 0;

    let x_len = graph.len() as i8;
    let y_len = graph[0].len() as i8;
    while let Some((a, b)) = dq.pop_front() {
        for dir in DIR {
            let nx = a + dir.0;
            let ny = b + dir.1;
            if nx >= 0 && nx < x_len && ny >= 0 && ny < y_len && graph[nx as usize][ny as usize] == 0 {
                graph[nx as usize][ny as usize] = 2;
                cnt += 1;
                dq.push_back((nx, ny));
            }
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let mut read_buf = BufReader::new(io::stdin().lock());
    let mut write_buf = BufWriter::new(io::stdout().lock());
    let mut buf_to_string = String::new();

    if let (Ok(n), Ok(m)) = read_line_to_nums!(read_buf, buf_to_string, usize) {
        let (susceptible_zone_idxs, map_table, epicenters) = read_to_tables!(read_buf, buf_to_string, n, m, i8);

        let voids_len = susceptible_zone_idxs.len();
        let mut min_outbreaks = voids_len as i8 - 3;

        for i in 0..voids_len-2 {
            for j in i+1..voids_len-1 {
                for k in j+1..voids_len {
                    let mut map = map_table.clone();
                    let mut outbreaks = epicenters.clone();
                    let iy = susceptible_zone_idxs[i] as usize % m;
                    let ix = susceptible_zone_idxs[i] as usize / m;
                    let jy = susceptible_zone_idxs[j] as usize % m;
                    let jx = susceptible_zone_idxs[j] as usize / m;
                    let ky = susceptible_zone_idxs[k] as usize % m;
                    let kx = susceptible_zone_idxs[k] as usize / m;
                    map[ix][iy] = 1;
                    map[jx][jy] = 1;
                    map[kx][ky] = 1;

                    let mut cnt = 0;

                    cnt += _bfs(&mut map,  &mut outbreaks);

                    min_outbreaks = min_outbreaks.min(cnt);
                }
            }

        }
        writeln!(write_buf, "{}", voids_len as i8 - min_outbreaks - 3)?;
    }
    Ok(())
}