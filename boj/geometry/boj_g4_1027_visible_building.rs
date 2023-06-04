// https://www.acmicpc.net/problem/1027

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());
    let mut read = || input.next().unwrap().unwrap();
    let n = read().parse::<usize>().unwrap();
    let buildings = read().split_ascii_whitespace().take(n).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut cnt: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut max_gradient = f64::MIN;
        for j in (i + 1)..n {
            let h = buildings[j] - buildings[i];
            let w = j - i;
            let g = h as f64 / w as f64;

            if g <= max_gradient {
                continue;
            }
            max_gradient = g;
            cnt[i] += 1;
            cnt[j] += 1;
        }
    }

    writeln!(output, "{}", cnt.iter().max().unwrap())?;
    Ok(())
}

// fn main() -> io::Result<()> {
//     let mut input = io::stdin().lock().lines();
//     let mut output = BufWriter::new(io::stdout().lock());
//     let mut read = || input.next().unwrap().unwrap();
//     let n = read().parse::<usize>().unwrap();
//     let heights = read().split_ascii_whitespace().take(n).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
//
//     // Compute visible counts for each building
//     let visible_counts = (0..n).map(|i| {
//         let mut max_slope = f64::NEG_INFINITY;
//         let mut cnt = 0;
//
//         // Check buildings to the left
//         if i > 0 {
//             for j in (0..i).rev() {
//                 let inverse_slope =  (i - j) as f64 / (heights[i] - heights[j]) as f64;
//                 if inverse_slope > max_slope {
//                     max_slope = inverse_slope;
//                     cnt += 1;
//                 }
//             }
//         }
//
//         // Check buildings to the right
//         max_slope = f64::NEG_INFINITY;
//         for j in i+1..n {
//             let slope = (heights[j] - heights[i]) as f64 / (j - i) as f64;
//             if slope > max_slope {
//                 max_slope = slope;
//                 cnt += 1;
//             }
//         }
//         cnt
//
//     }).collect::<Vec<_>>();
//
//     writeln!(output, "{}", visible_counts.iter().max().unwrap())?;
//     Ok(())
// }