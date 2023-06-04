// https://www.acmicpc.net/problem/16958

use std::io::{self, prelude::*, BufReader, BufWriter};

#[derive(Clone)]
struct Point {
    v: i32,
    y: i32,
    x: i32,
}

impl Point {
    fn new(mut s: std::str::SplitAsciiWhitespace) -> Self {
        Self {
            v: s.next().unwrap().parse::<i32>().unwrap(),
            y: s.next().unwrap().parse::<i32>().unwrap(),
            x: s.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

fn dist(a: &Point, b: &Point) -> i32 {
    (a.y - b.y).abs() + (a.x - b.x).abs()
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    let mut input = String::new();
    input.clear();
    reader.read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let t = iter.next().unwrap().parse::<i32>().unwrap();

    let mut d = vec![Point { v: 0, y: 0, x: 0 }; n + 1];
    for i in 1..=n {
        input.clear();
        reader.read_line(&mut input)?;
        d[i] = Point::new(input.split_ascii_whitespace());
    }

    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        if d[i].v == 1 {
            continue;
        }
        dp[i] = i32::MAX;
        for j in 1..=n {
            if i == j || d[j].v == 0 {
                continue;
            }
            let d = dist(&d[i], &d[j]);
            if dp[i] > d {
                dp[i] = d;
            }
        }
        if dp[i] == i32::MAX {
            dp[i] = 0;
        }
    }

    input.clear();
    reader.read_line(&mut input)?;
    let m = input.trim().parse::<usize>().unwrap();
    for _ in 0..m {
        input.clear();
        reader.read_line(&mut input)?;
        let mut iter = input.split_whitespace();
        let a = iter.next().unwrap().parse::<usize>().unwrap();
        let b = iter.next().unwrap().parse::<usize>().unwrap();
        let d_ab = dist(&d[a], &d[b]);
        let t_ab = dp[a] + t + dp[b];
        writeln!(writer, "{}", if d_ab < t_ab { d_ab } else { t_ab })?;
    }

    Ok(())
}

// // 첫번째 시도
// use std::{
//     io::{self, prelude::*, BufWriter},
// };
//
// fn main() -> io::Result<()> {
//     let mut input = io::stdin().lock().lines().map(|line| line.unwrap());
//     let mut output = BufWriter::new(io::stdout().lock());
//
//     let mut line = || input.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
//     let first = line();
//     let (n, t) = (first[0], first[1]);
//
//     let mut t_city = Vec::new();
//     let mut n_city = Vec::new();
//     let mut cities = vec![(0usize, 0i32, 0i32); n+1];
//
//     for i in 0..n {
//         let line = line();
//         cities[i+1] = (line[0], line[1] as i32, line[2] as i32);
//
//         if line[0] == 1 {
//             t_city.push(i+1);
//         } else {
//             n_city.push(i+1);
//         }
//     }
//
//     let m = line()[0];
//     for _ in 0..m {
//         let line = line();
//         let (a, b) = (line[0], line[1]);
//         let mut min = dist(cities[a], cities[b]);
//
//         if cities[a].0 == 1 && cities[b].0 == 1 {
//             min = usize::min(min, t)
//         } else if cities[a].0 == 1 || cities[b].0 == 1 {
//             let c = if cities[a].0 == 1 { b } else { a };
//             for i in &t_city {
//                 min = usize::min(min, dist(cities[*i], cities[c]) + t)
//             }
//         }
//
//         writeln!(output, "{}", min)?;
//     }
//
//     Ok(())
// }
//
// fn dist(a: (usize, i32, i32), b: (usize, i32, i32)) -> usize {
//     ((a.1 - b.1).abs() + (a.2 - b.2).abs()) as usize
// }

// // 두번째 시도
// use std::io::{self, BufRead, BufReader, BufWriter, Write};
//
// const INF: usize = 10_000;
//
// fn main() -> io::Result<()> {
//     let mut reader = BufReader::new(io::stdin().lock());
//     let mut writer = BufWriter::new(io::stdout().lock());
//     let mut input = String::new();
//     reader.read_line(&mut input)?;
//     let mut iter = input.split_whitespace();
//     let n = iter.next().unwrap().parse::<usize>().unwrap();
//     let t = iter.next().unwrap().parse::<usize>().unwrap();
//
//     let mut teleportable = vec![0usize; n];
//     let mut cities = vec![(0i32, 0i32); n];
//     let mut nodes = vec![vec![INF; n]; n];
//
//     for i in 0..n {
//         input.clear();
//         reader.read_line(&mut input)?;
//         let mut iter = input.split_whitespace();
//         teleportable[i] = iter.next().unwrap().parse().unwrap();
//         let row = iter.next().unwrap().parse().unwrap();
//         let col = iter.next().unwrap().parse().unwrap();
//         cities[i] = (row, col);
//     }
//
//     for i in 0..n {
//         for j in 0..n {
//             if i == j {
//                 nodes[i][j] = 0;
//             } else {
//                 let i_pos = cities[i];
//                 let j_pos = cities[j];
//                 let distance = (i_pos.0 - j_pos.0).abs() + (i_pos.1 - j_pos.1).abs();
//                 if teleportable[i] == 1 && teleportable[j] == 1 && t < distance as usize {
//                     nodes[i][j] = t;
//                 } else {
//                     nodes[i][j] = distance as usize;
//                 }
//             }
//         }
//     }
//
//     for k in 0..n {
//         for i in 0..n {
//             for j in 0..n {
//                 if nodes[i][j] > nodes[i][k] + nodes[k][j] {
//                     nodes[i][j] = nodes[i][k] + nodes[k][j];
//                 }
//             }
//         }
//     }
//
//     input.clear();
//     reader.read_line(&mut input)?;
//     let m = input.trim().parse::<usize>().unwrap();
//
//     for _ in 0..m {
//         input.clear();
//         reader.read_line(&mut input)?;
//         let mut iter = input.split_whitespace();
//         let start = iter.next().unwrap().parse::<usize>().unwrap() - 1;
//         let end = iter.next().unwrap().parse::<usize>().unwrap() - 1;
//         writeln!(writer, "{}", nodes[start][end])?;
//     }
//
//     writer.flush()?;
//
//     Ok(())
// }
