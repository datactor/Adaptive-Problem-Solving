// https://www.acmicpc.net/problem/4181

use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
};

type Point = (i32, i32);

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

macro_rules! read_to_vec {
    ($reader:expr, $iter:ident, $type:ty, $num:expr) => {
        {
            let mut input = String::new();
            let $iter: Vec<($type, $type)> = (0..$num).filter_map(|_| {
                input.clear();
                $reader.read_line(&mut input).expect("Failed to read line");
                let parts: Vec<&str> = input.split_ascii_whitespace().collect();
                if parts.len() < 3 {
                    return None;
                }
                let parsed_val1 = parts[0].parse::<$type>().expect("Failed to parse first value");
                let parsed_val2 = parts[1].parse::<$type>().expect("Failed to parse second value");
                let val3 = parts[2];
                if val3 == "Y" {
                    Some((parsed_val1, parsed_val2))
                } else {
                    None
                }
            }).collect();
            $iter
        }
    };
}

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let a = (p2.0 as i64 - p1.0 as i64) * (p3.1 as i64 - p1.1 as i64);
    let b = (p2.1 as i64 - p1.1 as i64) * (p3.0 as i64 - p1.0 as i64);
    if a > b { 1 }
    else if a < b { -1 }
    else { 0 }
}

fn dist(a: &Point, b: &Point) -> i64 {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;
    (dx * dx) + (dy * dy)
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    let n = read_input!(reader, buffer, usize);

    let mut points = read_to_vec!(reader, iter, i32, n);

    let min_idx = points.iter().enumerate().min_by_key(|&(_, point)| point.0).unwrap().0;
    points.swap(0, min_idx);

    let pivot = points[0];
    points.sort_unstable_by(|a, b| {
        let dir = ccw(&pivot, a, b);
        if dir != 0 {
            return dir.cmp(&0);
        }
        dist(&pivot, a).cmp(&dist(&pivot, b))
    });

    // exception handling
    let mut pt = points.len() - 1;
    let len = points.len();
    for _i in (1..len).rev() {
        if ccw(&points[0], &points[pt], &points[pt - 1]) == 0 {
            pt -= 1;
        } else {
            break;
        }
    }

    points[pt..].reverse();

    write!(writer, "{}\n{} {}\n", len, points[0].0, points[0].1)?;
    for i in (1..len).rev() {
        write!(writer, "{} {}\n", points[i].0, points[i].1)?;
    }

    Ok(())
}