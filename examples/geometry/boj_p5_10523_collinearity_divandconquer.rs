// https://www.acmicpc.net/problem/10523

use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    num::{ParseIntError as IE, ParseFloatError as FE},
    fmt,
};

type Point = (i32, i32);

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

fn are_points_collinear(p1: Point, p2: Point, p3: Point) -> bool {
    (p3.0 - p1.0) as i64 * (p2.1 - p1.1) as i64 == (p2.0 - p1.0) as i64 * (p3.1 - p1.1) as i64
}

fn find_lines(points: &[Point], target_len: usize) -> Vec<(Point, Point, usize)> {
    let mut lines = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let mut length = 2;

            for k in (j + 1)..points.len() {
                if are_points_collinear(points[i], points[j], points[k]) {
                    length += 1;
                }
            }

            if length >= target_len
                && lines.iter().all(|&(start_point, end_point, _)| {
                !(are_points_collinear(points[i], points[j], start_point) && are_points_collinear(points[j], start_point, end_point))
            })
            {
                lines.push((points[i], points[j], length));
            }
        }
    }

    lines
}

fn extend_lines(
    mut existing_lines: Vec<(Point, Point, usize)>,
    points: &[Point],
    target_len: usize,
) -> Vec<(Point, Point, usize)> {
    for &mut (start_point, end_point, ref mut length) in &mut existing_lines {
        for &point in points {
            if are_points_collinear(start_point, end_point, point) {
                *length += 1;
            }
        }
    }

    existing_lines.retain(|&(_, _, length)| length >= target_len);

    existing_lines
}

fn merge_lines(
    mut left_lines: Vec<(Point, Point, usize)>,
    right_lines: Vec<(Point, Point, usize)>,
) -> Vec<(Point, Point, usize)> {
    for &(start_point, end_point, length) in &right_lines {
        if left_lines.iter()
            .all(|&(p0, p1, _)| !(are_points_collinear(start_point, end_point, p0) && are_points_collinear(end_point, p0, p1)))
        {
            left_lines.push((start_point, end_point, length));
        }
    }

    left_lines
}

fn find_suitable_lines(points: &[Point], target_len: usize) -> Vec<(Point, Point, usize)> {
    if target_len < 6 {
        find_lines(points, target_len)
    } else {
        let midpoint = points.len() / 2;
        let half_length = target_len / 2 + if target_len % 2 == 0 { 0 } else { 1 };

        let left_points = &points[0..midpoint];
        let right_points = &points[midpoint..points.len()];

        let left_lines = find_suitable_lines(left_points, half_length);
        let right_lines = find_suitable_lines(right_points, half_length);

        let left_lines = extend_lines(left_lines, right_points, target_len);
        let right_lines = extend_lines(right_lines, left_points, target_len);

        merge_lines(left_lines, right_lines)
    }
}

fn is_possible(points: &[Point], target_len: usize) -> bool {
    if points.len() == 1 && target_len == 1 {
        return true;
    } else if points.len() == 1 && target_len > 1 {
        return false;
    }

    !find_suitable_lines(points, target_len).is_empty()
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let n = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    reader.read_line(&mut buffer)?;
    let percentage = buffer.trim().parse::<usize>().unwrap();

    let target_len = n * percentage / 100 + if n * percentage % 100 == 0 { 0 } else { 1 };

    let mut points: Vec<Point> = Vec::with_capacity(n);

    for _ in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        let mut iter = buffer.split_ascii_whitespace();
        points.push((iter.read(), iter.read()))
    }

    writeln!(writer, "{}",
        if is_possible(&points, target_len) {
            "possible"
        } else {
            "impossible"
        }
    )?;
    Ok(())
}