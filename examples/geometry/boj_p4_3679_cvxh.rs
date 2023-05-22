use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::Ordering,
};

type Point = (i32, i32, i32);

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

// fast
macro_rules! read_to_vec {
    ($reader:expr, $input:expr, $iter:ident, $type:ty) => {
        {
            $input.clear();
            $reader.read_line(&mut $input)?;
            let iter = $input.split_ascii_whitespace().skip(1);
            let mut num = -1;
            let $iter = iter.collect::<Vec<&str>>().chunks_exact(2).map(|chunk| {
                let parsed_val1 = chunk[0].parse::<$type>().unwrap();
                let parsed_val2 = chunk[1].parse::<$type>().unwrap();
                num += 1;
                (parsed_val1, parsed_val2, num)
            }).collect::<Vec<($type, $type, i32)>>();
            $iter
        }
    };
}

// eco memory
// macro_rules! read_to_vec {
//     ($reader:expr, $input:expr, $iter:ident, $type:ty) => {
//         {
//             $input.clear();
//             $reader.read_line(&mut $input)?;
//             let mut iter = $input.split_ascii_whitespace().skip(1);
//             let mut $iter = Vec::new();
//             while let (Some(val1), Some(val2)) = (iter.next(), iter.next()) {
//                 let parsed_val1 = val1.parse::<$type>().unwrap();
//                 let parsed_val2 = val2.parse::<$type>().unwrap();
//                 $iter.push((parsed_val1, parsed_val2));
//             }
//             $iter
//         }
//     };
// }

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let a = (p2.0 - p1.0) as i64 * (p3.1 - p1.1) as i64;
    let b = (p2.1 - p1.1) as i64 * (p3.0 - p1.0) as i64;
    if a > b { 1 }
    else if a < b { -1 }
    else { 0 }
}

fn angle(p1: &Point, p2: &Point) -> f64 {
    let dy = p2.1 as f64 - p1.1 as f64;
    let dx = p2.0 as f64 - p1.0 as f64;
    dy.atan2(dx)
}

fn dist(a: &Point, b: &Point) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx) as i64 + (dy * dy) as i64
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut input = String::new();

    let t = read_input!(reader, input, usize);
    for _ in 0..t {
        let mut points = read_to_vec!(reader, input, iter, i32);
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
        for _i in (1..points.len()).rev() {
            if ccw(&points[0], &points[pt], &points[pt - 1]) == 0 {
                pt -= 1;
            } else {
                break;
            }
        }

        points[pt..].reverse();

        // points.sort_unstable();
        // let pivot = points[0];
        // points[1..].sort_unstable_by(|a, b| {
        //     let angle1 = ccw(&pivot, a, b);
        //     if angle1 != 0 {
        //         return angle1.cmp(&0);
        //     }
        //     dist(&pivot, a).cmp(&dist(&pivot, b))
        // });
        // let mut pt = points.len() - 1;
        // while pt >= 1 && ccw(&points[0], &points[pt], &points[pt - 1]) == 0 {
        //     pt -= 1;
        // }
        // points[pt..].reverse();

        for p in points {
            write!(writer, "{} ", p.2)?;
        }
        write!(writer, "\n")?;
    }
    Ok(())
}