// use std::{
//     io::{self, prelude::*, BufReader, BufWriter},
//     str::{FromStr, SplitAsciiWhitespace},
//     num::{ParseIntError as IE, ParseFloatError as FE},
//     fmt,
// };
//
// trait Parser {
//     fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
// }
//
// impl<'a> Parser for SplitAsciiWhitespace<'a> {
//     fn read<T, E>(&mut self) -> T
//         where
//             T: FromStr<Err = E>,
//             E: fmt::Debug,
//     {
//         self.next().expect("EOF").parse().expect("Parse Error")
//     }
// }
//
// #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Point {
//     fn new(nums: (i32, i32)) -> Self {
//         Self { x: nums.0, y: nums.1 }
//     }
// }
//
// #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
// struct Line(Point, Point);
//
// impl Line {
//     fn is_intersect(&self, other: &Line) -> bool {
//         if (ccw(&self.0, &self.1, &other.0) * ccw(&self.0, &self.1, &other.1) < 0 ) ||
//             (ccw(&other.0, &other.1, &other.0) * ccw(&other.0, &other.1, &other.1) < 0) {
//             return true
//         } false
//     }
// }
//
// fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
//     let res = (p2.x - p1.x)*(p3.y - p1.y) - (p3.x - p1.x)*(p2.y - p1.y);
//     if res > 0 { 1 }
//     else if res < 0 { -1 }
//     else { 0 }
// }
//
// fn main() -> io::Result<()> {
//     let mut reader = BufReader::new(io::stdin().lock());
//     let mut writer = BufWriter::new(io::stdout().lock());
//     let mut input = String::new();
//     reader.read_line(&mut input)?;
//
//     let n = input.trim().parse::<usize>().unwrap()/2;
//     // println!("{}", n);
//
//     let mut lines = Vec::with_capacity(n);
//     let mut max = 0;
//     let mut min = 50;
//
//     let mut ans = Vec::with_capacity(n);
//
//     let mut qwer = vec![0; n];
//
//     for _ in 0..n {
//         input.clear();
//         reader.read_line(&mut input)?;
//         let mut iter = input.split_ascii_whitespace();
//         // let (edge1, coord1, edge2, coord2) = (iter.read(), iter.read(), iter.read(), iter.read());
//         let p1 = find_coord(iter.read(), iter.read());
//         let p2 = find_coord(iter.read(), iter.read());
//
//         lines.push(Line(Point::new(p1), Point::new(p2)));
//     }
//
//     for i in 0..n {
//         let mut sum = 0;
//         for j in i+1..n {
//             if lines[i].is_intersect(&lines[j]) {
//                 sum += 1;
//                 qwer[i] += 1;
//                 qwer[j] += 1;
//             }
//         }
//         max = i32::max(sum, max);
//         min = i32::min(sum, min);
//         ans.push(sum);
//     }
//
//     let mut f = 0;
//
//     for i in ans {
//         if i == min {
//             f += 1;
//         }
//     }
//
//     writeln!(writer, "{}", max)?;
//     writeln!(writer, "{}", f)?;
//     println!("{:?}", qwer);
//
//     Ok(())
// }
//
// fn find_coord(e: i32, c: i32) -> (i32, i32) {
//     let mut x = (0, 0);
//     if e == 1 {
//         x.0 = c;
//         x.1 = 51;
//     } else if e == 2 {
//         x.0 = c;
//         x.1 = 0;
//     } else if e == 3 {
//         x.0 = 0;
//         x.1 = 51 - c;
//     } else {
//         x.0 = 51;
//         x.1 = 51 - c;
//     }
//     x
// }

use std::cmp::max;
use std::io::{stdin, stdout, BufWriter, Write};

type P = (i32, i32);
type PP = ((i32, i32), (i32, i32));

fn cw(a: P, b: P) -> bool {
    if a.0 < b.0 {
        false
    } else if a.0 > b.0 {
        true
    } else if a.0 < 3 {
        a.1 > b.1
    } else {
        a.1 < b.1
    }
}

fn check(a: usize, b: usize, point: &[PP]) -> bool {
    let p1 = point[a].0;
    let p2 = point[a].1;
    let p3 = point[b].0;
    let p4 = point[b].1;
    let c13 = cw(p1, p3);
    let c23 = cw(p2, p3);
    let c14 = cw(p1, p4);
    let c24 = cw(p2, p4);
    (c13 ^ c23) ^ (c14 ^ c24)
}

fn convert(a: i32) -> i32 {
    match a {
        3 => 1,
        2 => 2,
        1 => 4,
        _ => 3,
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    let n = n / 2;
    let mut point: Vec<PP> = Vec::new();

    for _ in 0..n {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (mut a1, b1, mut a2, b2) = (nums[0], nums[1], nums[2], nums[3]);
        a1 = convert(a1);
        a2 = convert(a2);
        point.push(((a1, b1), (a2, b2)));
    }

    let mut ret = 0;
    let mut ans = 0;
    for i in 0..n {
        let mut tmp = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            if check(i as usize, j as usize, &point) {
                tmp += 1;
            }
        }
        ret += tmp;
        ans = max(ans, tmp);
    }

    let mut output = BufWriter::new(stdout());
    writeln!(output, "{}", ret / 2).unwrap();
    writeln!(output, "{}", ans).unwrap();
}