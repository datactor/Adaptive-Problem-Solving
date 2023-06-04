// https://www.acmicpc.net/problem/2162

use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    num::{ParseIntError as IE, ParseFloatError as FE},
    fmt,
};

trait Parser {
    fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
}

impl<'a> Parser for SplitAsciiWhitespace<'a> {
    fn read<T, E>(&mut self) -> T
        where
            T: FromStr<Err = E>,
            E: fmt::Debug,
    {
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

type Point = (i32, i32);

#[derive(Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }
}

fn direction(a: Point, b: Point, c: Point) -> i32 {
    let dxab = b.0 - a.0;
    let dxac = c.0 - a.0;
    let dyab = b.1 - a.1;
    let dyac = c.1 - a.1;

    if dxab * dyac < dyab * dxac { 1 }
    else if dxab * dyac > dyab * dxac { -1 }
    else if dxab == 0 && dyab == 0 { 0 }
    else if dxab * dxac < 0 || dyab * dyac < 0 { -1 }
    else if dxab * dxab + dyab * dyab >= dxac * dxac + dyac * dyac { 0 }
    else { 1 }
}

fn intersection(l1: Line, l2: Line) -> bool {
    direction(l1.p1, l1.p2, l2.p1) * direction(l1.p1, l1.p2, l2.p2) <= 0 &&
        direction(l2.p1, l2.p2, l1.p1) * direction(l2.p1, l2.p2, l1.p2) <= 0
}

// 경로 압축으로 노드가 직접 루트를 가리키도록 트리를 압축.
fn find_parent(parent: &mut Vec<Option<usize>>, x: usize) -> usize {
    match parent[x] {
        None => x,
        Some(p) => {
            let root = find_parent(parent, p);
            parent[x] = Some(root);  // Path compression
            root
        }
    }
}

fn union_parent(parent: &mut Vec<Option<usize>>, a: usize, b: usize) {
    let a = find_parent(parent, a);
    let b = find_parent(parent, b);
    if a != b {
        if a < b {
            parent[b] = Some(a);
        } else {
            parent[a] = Some(b);
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let n = buffer.trim().parse::<usize>().unwrap();
    let mut parent: Vec<Option<usize>> = vec![None; n];
    let mut lines_vec = Vec::with_capacity(n);

    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        let mut iter = buffer.split_ascii_whitespace();
        let line = Line::new((iter.read(), iter.read()), (iter.read(), iter.read()));

        lines_vec.push(line);
    }

    for i in 0..n {
        for j in i+1..n {
            if intersection(lines_vec[i], lines_vec[j]) {
                union_parent(&mut parent, i, j);
            }
        }
    }

    let mut counter = vec![0; n];
    for i in 0..n {
        let p = find_parent(&mut parent, i);
        counter[p] += 1;
    }

    let groups = counter.iter().filter(|&&x| x > 0).count();
    let max = *counter.iter().max().unwrap_or(&0);

    writeln!(writer, "{}\n{}", groups, max)?;
    Ok(())
}