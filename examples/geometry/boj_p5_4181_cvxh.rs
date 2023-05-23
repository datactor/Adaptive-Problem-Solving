use std::{
    io::{self, Write, BufRead, BufReader, BufWriter},
    cmp::Ordering,
};

type Point = (i64, i64);

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


// // fast
// macro_rules! read_to_vec {
//     ($reader:expr, $input:expr, $iter:ident, $type:ty, $num:expr) => {
//         {
//             $input.clear();
//             $reader.read_line(&mut $input)?;
//             let iter = $input.split_ascii_whitespace();
//             let $iter = iter.collect::<Vec<&str>>().chunks_exact(3).filter_map(|chunk| {
//                 let parsed_val1 = chunk[0].parse::<$type>().unwrap();
//                 let parsed_val2 = chunk[1].parse::<$type>().unwrap();
//                 if chunk[2] == "Y" {
//                     Some((parsed_val1, parsed_val2))
//                 } else {
//                     None
//                 }
//             }).collect::<Vec<($type, $type)>>();
//             $iter
//         }
//     };
// }

fn ccw(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let a = (p2.0 - p1.0) * (p3.1 - p1.1);
    let b = (p2.1 - p1.1) * (p3.0 - p1.0);
    if a > b { 1 }
    else if a < b { -1 }
    else { 0 }
}
//
// fn dist(a: &Point, b: &Point) -> i64 {
//     let dx = a.0 - b.0;
//     let dy = a.1 - b.1;
//     (dx * dx) + (dy * dy)
// }

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    let n = read_input!(reader, buffer, usize);

    let mut points = read_to_vec!(reader, iter, i64, n);

    points.sort_unstable_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let pivot = points[0];

    points[1..].sort_unstable_by(|a, b| {
        let order = ccw(&pivot, a, b).cmp(&0);
        match order {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => a.0.cmp(&b.0),
        }
    });

    let mut answer = vec![points[0]];
    for &point in &points[1..] {
        while answer.len() >= 2
            && ccw(&answer[answer.len() - 2], answer.last().unwrap(), &point) < 0
        {
            answer.pop();
        }
        answer.push(point);
    }

    write!(writer, "{}\n", points.len())?;
    for p in points {
        write!(writer, "{} {}\n", p.0, p.1)?;
    }

    Ok(())
}