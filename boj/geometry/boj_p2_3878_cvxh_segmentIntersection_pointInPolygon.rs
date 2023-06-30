// https://www.acmicpc.net/problem/3878

use std::{
    io::{self, Write, BufRead, BufWriter},
    error::Error,
    cmp::{min, max},
};

type Point = (i32, i32);

fn ccw(a: Point, b: Point, c: Point) -> i32 {
    let cross_ab_ac = (a.0 - c.0) * (b.1 - c.1);
    let cross_ba_bc = (a.1 - c.1) * (b.0 - c.0);
    if cross_ab_ac > cross_ba_bc { 1 }
    else if cross_ab_ac < cross_ba_bc { -1 }
    else { 0 }
}

fn get_cvxh(points: Vec<Point>, n: usize) -> Vec<Point> {
    let mut points = points;
    points.sort_unstable();
    if n <= 1 {
        return points;
    }
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    for i in 0..n {
        let j = n - 1 - i;
        while lower.len() >= 2 && ccw(lower[lower.len() - 2], lower[lower.len() - 1], points[i]) <= 0 {
            lower.pop();
        }
        lower.push(points[i]);
        while upper.len() >= 2 && ccw(upper[upper.len() - 2], upper[upper.len() - 1], points[j]) <= 0 {
            upper.pop();
        }
        upper.push(points[j]);
    }
    let mut res = lower;
    res.pop();
    res.append(&mut upper);
    res.pop();
    res
}

// fn get_cvxh(points: &mut Vec<Point>, n: usize) -> Vec<Point> {
//     // Graham's Scan
//     let min_idx = points.iter().enumerate().min_by_key(|&(_, (x, _))| x).unwrap().0;
//     points.swap(0, min_idx);
//     let pivot = points[0];
//     points[1..].sort_unstable_by(|p1, p2| {
//         let c = ccw(pivot, *p1, *p2);
//         if c > 0 { Ordering::Less }
//         else if c < 0 { Ordering::Greater }
//         else if dist(&pivot, &p1) < dist(&pivot, &p2) { Ordering::Less }
//         else { Ordering::Greater }
//     });
//
//     // get cvxh
//     let mut black_cvxh: Vec<(i32, i32)> = vec![];
//     for i in 0..n {
//         while black_cvxh.len() >= 2 && ccw(black_cvxh[black_cvxh.len() - 2], *black_cvxh.last().unwrap(), points[i]) <= 0 {
//             black_cvxh.pop();
//         }
//         black_cvxh.push(points[i]);
//     }
//
//     black_cvxh
// }
//
// fn dist(a: &Point, b: &Point) -> i32 {
//     (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
// }

fn cross_check(ab: [Point; 2], cd: [Point; 2]) -> bool {
    let abc = ccw(ab[0], ab[1], cd[0]);
    let abd = ccw(ab[0], ab[1], cd[1]);
    let cda = ccw(cd[0], cd[1], ab[0]);
    let cdb = ccw(cd[0], cd[1], ab[1]);
    if abc * abd <= 0 && cda * cdb <= 0 {
        if abc * abd == 0 && cda * cdb == 0 {
            let ab_mn_mx = [
                (min(ab[0].0, ab[1].0), max(ab[0].0, ab[1].0)),
                (min(ab[0].1, ab[1].1), max(ab[0].1, ab[1].1))
            ];
            let cd_mn_mx = [
                (min(cd[0].0, cd[1].0), max(cd[0].0, cd[1].0)),
                (min(cd[0].1, cd[1].1), max(cd[0].1, cd[1].1))
            ];
            return ab_mn_mx[0].0 <= cd_mn_mx[0].1 && ab_mn_mx[1].0 <= cd_mn_mx[1].1
                && cd_mn_mx[0].0 <= ab_mn_mx[0].1 && cd_mn_mx[1].0 <= ab_mn_mx[1].1;
        } else {
            return true;
        }
    }
    false
}

macro_rules! linetup {
    ($lines:expr, $type:ty) => {
        {
            let line = $lines.next().unwrap().unwrap();
            let mut iter = line.split_ascii_whitespace().map(|s| s.parse::<$type>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lock().lines();
    let mut buf_writer = BufWriter::new(io::stdout().lock());

    let t: usize = lines.next().unwrap()?.parse()?;
    for _ in 0..t {
        let (n, m) = linetup!(lines, usize);

        let black: Vec<Point> = (0..n).map(|_| {
            linetup!(lines, i32)
        }).collect();

        let white: Vec<Point> = (0..m).map(|_| {
            linetup!(lines, i32)
        }).collect();

        let b_cvxh = get_cvxh(black, n);
        let w_cvxh = get_cvxh(white, m);
        let b_len = b_cvxh.len();
        let w_len = w_cvxh.len();

        if b_len < 3 && w_len < 3 {
            if cross_check([b_cvxh[0], b_cvxh[1 % b_len]], [w_cvxh[0], w_cvxh[1 % w_len]]) {
                writeln!(buf_writer, "NO")?;
                continue;
            } else {
                writeln!(buf_writer, "YES")?;
                continue;
            }
        }
        let mut ccw_b_cnt = [0, 0];
        let mut ccw_w_cnt = [0, 0];
        let mut is_splitable = true;
        for i in 0..b_len {
            let line_b = [b_cvxh[i], b_cvxh[(i + 1) % b_len]];
            for j in 0..w_len {
                let ccw_b = ccw(b_cvxh[i], w_cvxh[j], w_cvxh[(j + 1) % w_len]);
                let ccw_w = ccw(w_cvxh[j], b_cvxh[i], b_cvxh[(i + 1) % b_len]);
                if ccw_b > 0 { ccw_b_cnt[0] += 1; }
                else if ccw_b < 0 { ccw_b_cnt[1] += 1; }
                if ccw_w > 0 { ccw_w_cnt[0] += 1; }
                else if ccw_w < 0 { ccw_w_cnt[1] += 1; }
                let line_w = [w_cvxh[j], w_cvxh[(j + 1) % w_len]];
                if cross_check(line_b, line_w) {
                    is_splitable = false;
                    break;
                }
            }
            if !is_splitable { break; }
        }
        if ccw_b_cnt.iter().max().unwrap() >= &(b_len * w_len) || ccw_w_cnt.iter().max().unwrap() >= &(b_len * w_len) {
            is_splitable = false;
        }
        writeln!(buf_writer, "{}", if is_splitable {
            "YES"
        } else {
            "NO"
        })?
    }
    Ok(())
}
