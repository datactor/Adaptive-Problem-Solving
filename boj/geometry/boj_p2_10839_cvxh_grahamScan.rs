use std::{
    error::Error,
    io::{self, BufWriter, Read, Write},
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn next<T>(&mut self) -> Result<T, Box<dyn Error>>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.input
            .next()
            .ok_or("EOF")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn ccw(&self, p1: &Point, p2: &Point) -> i32 {
        let cross_product = (p1.x - self.x) as i64 * (p2.y - self.y) as i64
            - (p1.y - self.y) as i64 * (p2.x - self.x) as i64;
        cross_product.signum() as i32
    }

    // fn squared_distance_to(&self, other: &Point) -> i64 {
    //     let dx = (self.x - other.x) as i64;
    //     let dy = (self.y - other.y) as i64;
    //     dx * dx + dy * dy
    // }
}

// struct Cvxh {
//     points: Vec<Point>,
// }

// impl Cvxh {
//     fn find_pivot(points: &mut Vec<Point>) -> Point {
//         let min_idx = points
//             .iter()
//             .enumerate()
//             .min_by_key(|&(_, point)| (point.x, point.y))
//             .unwrap()
//             .0;
//         points.swap(0, min_idx);
//         points[0]
//     }

//     fn sort_by_polar_angle(pivot: Point, points: &mut Vec<Point>) {
//         points[1..].sort_unstable_by(|p1, p2| {
//             let orientation = pivot.ccw(p1, p2);
//             if orientation != 0 {
//                 0.cmp(&orientation)
//             } else {
//                 pivot
//                     .squared_distance_to(p1)
//                     .cmp(&pivot.squared_distance_to(p2))
//             }
//         });
//     }

//     fn build_cvxh(points: &Vec<Point>) -> Vec<Point> {
//         let mut cvxh: Vec<Point> = Vec::new();
//         for &next_point in points {
//             while cvxh.len() >= 2
//                 && cvxh[cvxh.len() - 2].ccw(cvxh.last().unwrap(), &next_point) <= 0
//             {
//                 cvxh.pop();
//             }
//             cvxh.push(next_point);
//         }
//         cvxh
//     }

//     fn graham_scan_from_points(mut points: Vec<Point>) -> Vec<Point> {
//         let pivot = Self::find_pivot(&mut points);
//         Self::sort_by_polar_angle(points[0], &mut points);
//         Self::build_cvxh(&points)
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = BufWriter::new(io::stdout());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut scanner = Scanner::new(&buffer);
    let n = scanner.next::<usize>()?;
    let sorted_points = (0..n)
        .map(|_| {
            let x = scanner.next::<i32>()?;
            let y = scanner.next::<i32>()?;
            Ok(Point { x, y })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    let (mut start, mut end) = (scanner.next::<usize>()?, scanner.next::<usize>()?);
    if [
        (start as i32 - end as i32) % n as i32,
        (end as i32 - start as i32) % n as i32,
    ]
    .contains(&1)
    {
        write!(writer, "2\n{} {}", start, end)?;
        return Ok(());
    }

    let mut flag = false;
    if start == 0 || (end != 0 && start > end) {
        flag = true;
        std::mem::swap(&mut start, &mut end);
    }

    let mut cvxh = vec![start, (start + 1) % n];

    for mut i in (start + 1)..(start + n) {
        i %= n;
        while cvxh.len() >= 2 {
            if sorted_points[cvxh[cvxh.len() - 2]]
                .ccw(&sorted_points[cvxh[cvxh.len() - 1]], &sorted_points[i])
                >= 0
            {
                cvxh.pop();
            } else {
                break;
            }
        }
        cvxh.push(i);
        if i == end {
            break;
        }
    }

    writeln!(writer, "{}", cvxh.len())?;

    if flag {
        for idx in cvxh.iter().rev() {
            write!(writer, "{} ", idx)?;
        }
    } else {
        for idx in cvxh.iter() {
            write!(writer, "{} ", idx)?;
        }
    }

    Ok(())
}
