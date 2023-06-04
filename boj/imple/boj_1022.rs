use std::{
    error::Error,
    io::{self, prelude::*, BufWriter},
};

fn array(x: isize, y: isize) -> isize {
    let layer = x.abs().max(y.abs());
    let quarter = layer << 1;
    let cells = (quarter - 1).pow(2);
    match x {
        x if x == layer && y == layer => ((layer + 1 << 1) - 1).pow(2),
        x if x == layer => cells + (layer - y),
        x if y == -layer => cells + quarter + (layer - x),
        x if x == -layer => cells + 3 * quarter + (y - layer),
        x if y == layer => cells + 4 * quarter + (x - layer),
        _ => layer,
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input).unwrap();

    let mut v = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>())
        .flatten();
    let (r1, c1, r2, c2) = (
        v.next().unwrap(),
        v.next().unwrap(),
        v.next().unwrap(),
        v.next().unwrap(),
    );
    let max = std::cmp::max(
        array(c1, r1).max(array(c1, r2)),
        array(c2, r1).max(array(c2, r2)),
    );
    let len = max.to_string().len();
    for r in r1..=r2 {
        for c in c1..=c2 {
            write!(output, "{1:0$} ", len, array(c, r))?;
        }
        writeln!(output)?;
    }

    Ok(())
}

//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////

// use std::{
//     io::{self, prelude::*, BufWriter},
//     error::Error,
// };
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let mut output = BufWriter::new(io::stdout().lock());
//     io::stdin().read_line(&mut input)?;
//
//     let dx = [-1, 0, 1, 0];
//     let dy = [0, 1, 0, -1];
//
//     let mut v = input.split_ascii_whitespace().map(
//     |s| s.parse::<i32>()).flatten();
//     let (r1, c1, r2, c2) =
//     (v.next().unwrap(), v.next().unwrap(), v.next().unwrap(), v.next().unwrap());
//
//     let mut array = vec![vec![0; (c2 - c1 + 1) as usize]; (r2 - r1 + 1) as usize];
//     let mut total = (c2-c1+1) * (r2-r1+1);
//     let mut d = 1;
//     let (mut x, mut y, mut cnt, mut l, mut m) = (0, 0, 1, 1, 0);
//     let mut tmp_d: i32 = 1;
//
//     while total > 0 {
//         for _ in 0..2 {
//             for _ in 0..l {
//                 if r1 <= x && x <= r2 && c1 <= y && y <= c2 {
//                     array[(x-r1) as usize][(y-c1) as usize] = cnt;
//                     total -= 1;
//                     m = cnt;
//                 }
//                 x += dx[d];
//                 y += dy[d];
//                 cnt += 1;
//             } tmp_d = (tmp_d-1) % 4;
//             d = tmp_d as usize;
//             if tmp_d < 0 {
//                 d = (tmp_d + 4) as usize;
//             }
//         } l += 1
//     }
//     let max_len = m.to_string().len();
//     for i in 0..r2-r1+1 {
//         for j in 0..c2-c1+1 {
//             write!(output, "{1:0$} ", max_len, array[i as usize][j as usize])?;
//         } write!(output, "\n")?;
//     }
//     Ok(())
// }
