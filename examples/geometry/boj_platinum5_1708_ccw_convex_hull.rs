// https://www.acmicpc.net/problem/1708

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut points = input.skip(1).map(|line|
        {
            let string = line.unwrap();
            let (x, y) = string.split_once(' ').unwrap();
            let (x, y) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
            (x, y)
        }
    ).collect::<Vec<(i32, i32)>>();

    // Sort points in ascending order of x coordinate, and then y coordinate
    points.sort_by(|a, b| a.cmp(b));
//
//     // Calculate convex hull in counter-clockwise direction
//     let mut result = convex_hull(&mut points);
//
//     // Reverse order of points and calculate convex hull in clockwise direction
//     points.reverse();
//     result += convex_hull(&mut points);
//
//     // Subtract 2 from the result to remove duplicate points in the convex hull
//     writeln!(output, "{}", result - 2)?;
//
//     Ok(())
// }
//
// // Returns true if the three given points are in counter-clockwise order
// fn ccw(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> bool {
//     (x2 as i64 - x1 as i64) * (y3 as i64 - y1 as i64) - (y2 as i64 - y1 as i64) * (x3 as i64 - x1 as i64) > 0
// }
//
// // Calculates the convex hull of a set of points in ccw order
// fn convex_hull(points: &mut Vec<(i32, i32)>) -> usize {
//     let mut hull: Vec<(i32, i32)> = Vec::new();
//     for &p3 in points.iter() {
//         while hull.len() >= 2 {
//             let p1 = hull[hull.len() - 2];
//             let p2 = hull[hull.len() - 1];
//             if ccw(p1.0, p1.1, p2.0, p2.1, p3.0, p3.1) {
//                 break;
//             }
//             hull.pop();
//         }
//         hull.push(p3);
//     }
//     return hull.len();
// }

    let n = points.len();
    let mut upper_hull: Vec<(i32, i32)> = Vec::new();
    let mut lower_hull: Vec<(i32, i32)> = Vec::new();

    // Calculate convex hull
    for u in 0..n {
        let l = n - u - 1;
        while upper_hull.len() >= 2 {
            let p1 = upper_hull[upper_hull.len() - 2];
            let p2 = upper_hull[upper_hull.len() - 1];
            if ccw((p1.0, p1.1), (p2.0, p2.1), (points[u].0, points[u].1)) {
                break;
            }
            upper_hull.pop();
        }
        upper_hull.push(points[u]);

        while lower_hull.len() >= 2 {
            let p1 = lower_hull[lower_hull.len() - 2];
            let p2 = lower_hull[lower_hull.len() - 1];
            if ccw((p1.0, p1.1), (p2.0, p2.1), (points[l].0, points[l].1)) {
                break;
            }
            lower_hull.pop();
        }
        lower_hull.push(points[l]);
    }

    // Remove duplicate point from upper hull and lower hull
    upper_hull.pop();
    lower_hull.pop();

    // Combine upper and lower hull to get the convex hull
    writeln!(output, "{}", upper_hull.len() + lower_hull.len())?;

    Ok(())
}

// Returns true if the three given points are in counter-clockwise order
// 문제의 조건 중 내각이 180도 이하인지 확인
fn ccw(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    (p2.0 as i64 - p1.0 as i64) * (p3.1 as i64 - p1.1 as i64) - (p2.1 as i64 - p1.1 as i64) * (p3.0 as i64 - p1.0 as i64) > 0
}