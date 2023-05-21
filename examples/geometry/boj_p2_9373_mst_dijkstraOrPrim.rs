// https://www.acmicpc.net/problem/9373
// O(n^2)

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    collections::BinaryHeap,
    cmp::Ordering,
};

macro_rules! read_input {
    ($reader:expr, $input:expr, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let value = $input.trim().parse::<$type>().unwrap();
            value
        }
    };
}

macro_rules! read_input_iter {
    ($reader:expr, $input:expr, $iter:ident) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let $iter = $input.split_ascii_whitespace();
            $iter
        }
    };
}

macro_rules! parse_iter_values {
    ($iter:expr, $($var:ident: $type:ty),*) => {
        {
            let values = ($($iter.next().unwrap().parse::<$type>().unwrap()),*);
            values
        }
    };
}

type Circle = (i32, i32, i32);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct State {
    cost: ReverseF64,
    position: usize,
}

#[derive(Copy, Clone, Debug)]
struct ReverseF64(f64);

impl PartialEq for ReverseF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ReverseF64 {}

impl PartialOrd for ReverseF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for ReverseF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// O(m log n) prim
fn solve_prim(w: i32, n: usize, a: &[Circle], dists: &mut Vec<f64>, heap: &mut BinaryHeap<State>) -> f64 {
    let mut conn: Vec<bool> = vec![false; n + 1];

    dists.push(w as f64);
    heap.push(State { cost: ReverseF64(dists[n]), position: n });

    let mut ans: f64 = 0.0;
    while let Some(State { cost: _, position }) = heap.pop() {
        if conn[position] {
            continue;
        }
        conn[position] = true;
        ans = ans.max(dists[position]);
        if position == n {
            return ans / 2.0;
        }

        for i in 0..n {
            if !conn[i] {
                let dx = a[position].0 - a[i].0;
                let dy = a[position].1 - a[i].1;
                let d = ((dx as f64 * dx as f64) + (dy as f64 * dy as f64)).sqrt() - a[position].2 as f64 - a[i].2 as f64;
                if d < dists[i] {
                    dists[i] = d;
                    heap.push(State { cost: ReverseF64(d), position: i });
                }
            }
        }
        let d = (w - a[position].0 - a[position].2) as f64;
        if d < dists[n] {
            dists[n] = d;
            heap.push(State { cost: ReverseF64(d), position: n });
        }
    }
    0.0
}

// O(n^2) Dijkstra's Algorithm
fn solve_dijkstra(w: i32, n: usize, circles: &[Circle], dist: &mut Vec<f64>) -> f64 {
    let mut conn: Vec<bool> = vec![false; n + 1];

    dist.push(w as f64);

    let mut ans: f64 = 0.0;
    loop {
        let mut min_idx_wrapper = None;
        for i in 0..=n {
            if !conn[i] && (min_idx_wrapper.is_none() || dist[i] < dist[min_idx_wrapper.unwrap()]) {
                min_idx_wrapper = Some(i);
            }
        }

        if let Some(min_idx) = min_idx_wrapper {
            conn[min_idx] = true;
            ans = ans.max(dist[min_idx]);

            if min_idx == n {
                return ans / 2.0;
            }

            for i in 0..n {
                if !conn[i] {
                    let dx = circles[min_idx].0 - circles[i].0;
                    let dy = circles[min_idx].1 - circles[i].1;
                    let d = ((dx as f64 * dx as f64) + (dy as f64 * dy as f64)).sqrt() - circles[min_idx].2 as f64 - circles[i].2 as f64;
                    dist[i] = dist[i].min(d);
                }
            }

            dist[n] = dist[n].min((w - circles[min_idx].0 - circles[min_idx].2) as f64);
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();
    let t = read_input!(reader, input, usize);

    for _ in 0..t {
        let w = read_input!(reader, input, i32);
        let n = read_input!(reader, input, usize);

        let mut circles: Vec<Circle> = Vec::with_capacity(n);
        let mut dists: Vec<f64> = vec![];
        let mut heap = vec![];

        for i in 0..n {
            let mut iter = read_input_iter!(reader, input, iter);
            let (x, y, r) = parse_iter_values!(iter, x: i32, y: i32, r: i32);
            circles.push((x, y, r));
            let dist = (x-r) as f64;
            dists.push(dist);
            heap.push(State { cost: ReverseF64(dist), position: i });
        };
        let mut heap = BinaryHeap::from(heap);

        // let _ans_prim = solve_prim(w, n, &circles, &mut dists, &mut heap);
        let ans_dijkstra = solve_dijkstra(w, n, &circles, &mut dists);
        write!(writer, "{:.6}\n", ans_dijkstra)?;
    }
    Ok(())
}