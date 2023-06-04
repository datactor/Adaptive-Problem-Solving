use std::collections::HashSet;
use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut buffer = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();
    let n = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut hashset = HashSet::with_capacity(n);
    for line in lines.by_ref().take(n) {
        hashset.insert(line);
    }
    let result = lines.fold(0, |s, l| s + hashset.contains(&l) as usize);
    writeln!(output, "{}", result).unwrap();
}

// Refcell로 내부 가변성으로 풀어보기, 시간복잡도 계산해보기

// use std::io::{prelude::*, BufWriter, self};
// use std::str::from_utf8;
//
// // struct Scanner<'a> {
// //     inner: std::str::SplitAsciiWhitespace<'a>,
// // }
// //
// // impl<'a> Scanner<'a> {
// //     fn new(buf: &'a str) -> Scanner<'a> {
// //         Scanner {
// //             inner: buf.split_ascii_whitespace(),
// //         }
// //     }
// //     fn read(&mut self) -> &[u8] {
// //         self.inner.next().unwrap().as_bytes()
// //     }
// // }
//
//
// fn main() {
//     let mut output = BufWriter::new(io::stdout().lock());
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).unwrap();
//     let x = buffer.split_ascii_whitespace().map(
//         |s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
//     let (n, m) = (x[0], x[1]);
//
//     buffer.clear();
//     io::stdin().read_to_string(&mut buffer).unwrap();
//     let mut v = buffer.split_ascii_whitespace().map(
//         |s| (s.as_bytes(), s.as_bytes().len())).collect::<Vec<_>>();
//
//     // v[0..n].sort_by_key(|&(_, a)| a);
//     // v[n..n+m].sort_by_key(|&(_, a)| a);
//     let mut result = 0;
//
//     for i in n..m+n {
//         if v[0..n].contains(&v[i]) {
//             result += 1;
//     }
//
//     // for i in 0..n {
//     //     let mut tmp = n;
//     //     while tmp < m+n {
//     //         if v[i].1 < v[tmp].1 {
//     //             break
//     //         } else if v[i] == v[tmp] {
//     //             result += 1;
//     //         }
//     //         tmp += 1;
//     //     }
//
//     // for i in 0..n {
//     //     let mut tmp = n;
//     //     while tmp < m+n {
//     //         if v[i][0] < v[tmp][0] {
//     //             break
//     //         } else if v[i] == v[tmp] {
//     //             result += 1;
//     //         }
//     //         tmp += 1;
//     //     }
//
//         // for j in n..n+m {
//         //     if v[i] == v[j] {
//         //         result += 1;
//         //     }
//         // }
//     }
//     println!("{}", result);
//
//     // println!("{:?}", v);
//
//     // let mut scanner = Scanner::new(&buffer);
//     // solve(&mut scanner, &mut output);
//     // println!("{}", n);
// }
//
// // fn solve<W: Write>(scanner: &mut Scanner, output: &mut BufWriter<W>) {
// //     let n_set = scanner.read();
// //     let mut n = 0;
// //     let mut multi = 1;
// //     for i in n_set.iter().rev() {
// //         let x = (i-48) * multi;
// //         n += x;
// //         multi *= 10;
// //     }
// //
// //     let m_set = scanner.read();
// //     let mut m = 0;
// //     let mut multi = 1;
// //     for i in m_set.iter().rev() {
// //         let x = (i-48) * multi;
// //         m += x;
// //         multi *= 10;
// //     }
// //
// //     println!("{}, {}", n, m);
// //     // println!("{:?}", scanner.read());
// //     // let mut s = Vec::new();
// //     while true {
// //         let a = scanner.read();
// //         s.push(a.clone());
// //     }
// //
// //     // let mut hand = Vec::new();
// //     // for _ in 0..m {
// //     //     hand.push(scanner.read());
// //     // }
// //     let s = (0..n).map(
// //         |_| scanner.read()).collect::<Vec<_>>();
// //
// //     // let hand = (0..m).map(
// //     //     |a| scanner.read()).collect::<Vec<_>>();
// //
// //     println!("{:?}", s);
// //
// //
// // }
