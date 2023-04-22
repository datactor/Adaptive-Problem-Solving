// https://www.acmicpc.net/problem/22942
// O(2n lg 2n)

use std::io::{self, Write, BufRead, BufWriter};

fn main() -> io::Result<()> {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut lines = io::stdin().lock().lines().map(|line| line.unwrap());
    let n = lines.next().unwrap().trim().parse::<usize>().unwrap();

    let mut circles: Vec<(i32, usize)> = Vec::with_capacity(n * 2);
    for i in 0..n {
        let line = lines.next().unwrap();
        let mut iter = line.split_ascii_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let r = iter.next().unwrap().parse::<i32>().unwrap();
        circles.push((x - r, i));
        circles.push((x + r, i));
    }
    circles.sort();

    let mut stack: Vec<(i32, usize)> = Vec::new();
    for c in circles {
        if let Some((_, prev)) = stack.last() {
            if *prev == c.1 {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    writeln!(output, "{}", if stack.is_empty() { "YES" } else { "NO" })?;
    Ok(())
}

// // 1차 시도
// use std::{
//     io::{self, prelude::*, BufReader, BufWriter},
//     collections::VecDeque,
// };
//
// #[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
// struct Circle {
//     s: i32,
//     e: i32,
// }
//
// impl Circle {
//     fn new(x: i32, r: i32) -> Self {
//         Self {
//             s: x - r,
//             e: x + r,
//         }
//     }
// }
//
// fn main() -> io::Result<()> {
//     let mut reader = BufReader::new(io::stdin().lock());
//     let mut writer = BufWriter::new(io::stdout().lock());
//     let mut input = String::new();
//     reader.read_line(&mut input)?;
//     let n = input.trim().parse::<usize>().unwrap();
//
//     let mut circles = Vec::new();
//     for i in 0..n {
//         input.clear();
//         reader.read_line(&mut input)?;
//         let mut iter = input.split_ascii_whitespace();
//         let mut next = || iter.next().unwrap().parse::<i32>().unwrap();
//         let (x, r) = (next(), next());
//         let (s, e) = (x-r, x+r);
//         circles.push(Circle::new(x, r));
//     }
//
//     circles.sort();
//     circles.sort_by_key(|circles| circles.e);
//
//
//     // println!("{:?}", circles);
//     writeln!(writer, "{}", find_intersection(&circles, n))?;
//
//     Ok(())
// }
//
// fn find_intersection(circles: &Vec<Circle>, n: usize) -> &str {
//     for i in 0..n {
//         // let mut stack = VecDeque::new();
//         for j in i+1..n {
//             if circles[j].s <= circles[i].e {
//                 if circles[j].s >= circles[i].s {
//                     continue
//                 } else {
//                     // stack.push_back(circles[j]);
//                     return "YES"
//                 }
//             } else {
//                 // break;
//                 return "YES"
//             }
//         }
//         // while let Some(f_circle) = stack.pop_back() {
//         //     if f_circle.s > circles[i].e {
//         //         break;
//         //     }
//         // }
//     }
//     "NO"
// }

// // 2차 시도
// use std::io::{self, prelude::*, BufWriter, BufReader};
//
// #[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
// struct Circle {
//     x: i32,
//     r: i32,
// }
//
// impl Circle {
//     fn new(mut s: std::str::SplitAsciiWhitespace) -> Self {
//         Self {
//             x: s.next().unwrap().parse::<i32>().unwrap(),
//             r: s.next().unwrap().parse::<i32>().unwrap(),
//         }
//     }
//
//     fn is_overlap(&self, other: &Circle) -> bool {
//         let dist = (self.x - other.x).abs();
//         let r_sum = self.r + other.r;
//         let r_sub = (self.r - other.r).abs();
//         if r_sub <= dist && dist <= r_sum {
//             return true
//         }
//         false
//     }
// }
//
// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let stdout = io::stdout();
//     let mut input = BufReader::new(stdin.lock());
//     let mut output = BufWriter::new(stdout.lock());
//
//     let mut line = String::new();
//     input.read_line(&mut line)?;
//     let n = line.trim().parse::<usize>().unwrap();
//
//     let mut circles: Vec<Circle> = Vec::with_capacity(n);
//     for _ in 0..n {
//         line.clear();
//         input.read_line(&mut line)?;
//         circles.push(Circle::new(line.trim().split_ascii_whitespace()));
//     }
//
//     circles.sort_by_key(|circle| circle.x);
//     let mut active_circles: Vec<Circle> = Vec::new();
//     let mut overlap = false;
//
//     for i in 0..n {
//         let circle = &circles[i];
//         while let Some(active_circle) = active_circles.first() {
//             if active_circle.x + active_circle.r < circle.x {
//                 active_circles.remove(0);
//             } else {
//                 break;
//             }
//         }
//         for active_circle in &active_circles {
//             if circle.is_overlap(active_circle) {
//                 overlap = true;
//                 break;
//             }
//         }
//         if overlap {
//             break;
//         }
//         active_circles.push(*circle);
//         if i > 0 {
//             active_circles.sort_by_key(|circle| circle.r);
//         }
//     }
//
//     writeln!(output, "{}",
//              if overlap {
//                  "NO"
//              } else {
//                  "YES"
//              }
//     )?;
//     Ok(())
// }
//
// // 3차 시도 98% 실패
// use std::io::{self, prelude::*, BufWriter, BufReader};
// use std::collections::VecDeque;
//
// #[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
// struct Circle {
//     x: i32,
//     r: i32,
// }
//
// impl Circle {
//     fn new(mut s: std::str::SplitAsciiWhitespace) -> Self {
//         Self {
//             x: s.next().unwrap().parse::<i32>().unwrap(),
//             r: s.next().unwrap().parse::<i32>().unwrap(),
//         }
//     }
//
//     fn is_overlap(&self, other: &Circle) -> bool {
//         let dist = (self.x - other.x).abs();
//         let r_sum = self.r + other.r;
//         let r_sub = (self.r - other.r).abs();
//         // if (r_sub <= dist && dist <= r_sum) || r_sum == dist || r_sub == dist {
//         //     return true
//         // }
//         // false
//         if (r_sum < dist || dist < r_sub || dist == 0) && self.x+self.r != other.x-other.r && self.x-self.r != other.x+other.r {
//             return false
//         }
//         true
//     }
// }
//
// fn find_intersection(circles: &[Circle], n: usize) -> &str {
//     let mut stack: VecDeque<Circle> = VecDeque::with_capacity(n);
//     for i in 0..n {
//         while let Some(f_circle) = stack.back() {
//             if f_circle.x + f_circle.r < circles[i].x {
//                 stack.pop_back();
//             } else {
//                 break;
//             }
//         }
//         if let Some(f_circle) = stack.back() {
//             if circles[i].is_overlap(f_circle) {
//                 return "NO";
//             }
//         }
//         stack.push_back(circles[i]);
//     }
//     "YES"
// }
//
// fn main() -> io::Result<()> {
//     let mut input = BufReader::new(io::stdin().lock());
//     let mut output = BufWriter::new(io::stdout().lock());
//
//     let mut line = String::new();
//     input.read_line(&mut line)?;
//     let n = line.trim().parse::<usize>().unwrap();
//
//     let mut circles: Vec<Circle> = Vec::with_capacity(n);
//     for _ in 0..n {
//         line.clear();
//         input.read_line(&mut line)?;
//         circles.push(Circle::new(line.split_ascii_whitespace()));
//     }
//
//     circles.sort_by_key(|circle| circle.x);
//     writeln!(output, "{}", find_intersection(&circles, n))?;
//     Ok(())
// }
