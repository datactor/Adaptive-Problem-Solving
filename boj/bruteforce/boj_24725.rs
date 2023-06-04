// https://www.acmicpc.net/problem/24725

use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
};

const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let input = input.lines();

    let table: Vec<&[u8]> = input.skip(1).map(|s| s.trim().as_bytes()).collect();

    let (n, m) = (table.len(), table[0].len());

    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if table[i][j] == b'E' || table[i][j] == b'I' {
                ans += find(i, j, &table);
            }
        }
    }

    // // 8방향 단순 구현
    // for i in 0..n {
    //     for j in 0..m {
    //         if table[i][j] == b'E' || table[i][j] == b'I' {
    //             let (mut l, mut r, mut u, mut d) = (false, false, false, false);
    //             if i+3 < n {
    //                 d = true
    //             }
    //             if i >= 3 {
    //                 u = true
    //             }
    //             if j+3 < m {
    //                 r = true
    //             }
    //             if j >= 3 {
    //                 l = true
    //             }
    //
    //             if d {
    //                 if table[i+1][j] == b'N' || table[i+1][j] == b'S' {
    //                     if table[i+2][j] == b'F' || table[i+2][j] == b'T' {
    //                         if table[i+3][j] == b'P' || table[i+3][j] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if u {
    //                 if table[i-1][j] == b'N' || table[i-1][j] == b'S' {
    //                     if table[i-2][j] == b'F' || table[i-2][j] == b'T' {
    //                         if table[i-3][j] == b'P' || table[i-3][j] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if r {
    //                 if table[i][j+1] == b'N' || table[i][j+1] == b'S' {
    //                     if table[i][j+2] == b'F' || table[i][j+2] == b'T' {
    //                         if table[i][j+3] == b'P' || table[i][j+3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if l {
    //                 if table[i][j-1] == b'N' || table[i][j-1] == b'S' {
    //                     if table[i][j-2] == b'F' || table[i][j-2] == b'T' {
    //                         if table[i][j-3] == b'P' || table[i][j-3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if d && l {
    //                 if table[i+1][j-1] == b'N' || table[i+1][j-1] == b'S' {
    //                     if table[i+2][j-2] == b'F' || table[i+2][j-2] == b'T' {
    //                         if table[i+3][j-3] == b'P' || table[i+3][j-3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if d && r {
    //                 if table[i+1][j+1] == b'N' || table[i+1][j+1] == b'S' {
    //                     if table[i+2][j+2] == b'F' || table[i+2][j+2] == b'T' {
    //                         if table[i+3][j+3] == b'P' || table[i+3][j+3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if u && l {
    //                 if table[i-1][j-1] == b'N' || table[i-1][j-1] == b'S' {
    //                     if table[i-2][j-2] == b'F' || table[i-2][j-2] == b'T' {
    //                         if table[i-3][j-3] == b'P' || table[i-3][j-3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //
    //             if u && r {
    //                 if table[i-1][j+1] == b'N' || table[i-1][j+1] == b'S' {
    //                     if table[i-2][j+2] == b'F' || table[i-2][j+2] == b'T' {
    //                         if table[i-3][j+3] == b'P' || table[i-3][j+3] == b'J' {
    //                             ans += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    writeln!(output, "{}", ans)?;
    Ok(())
}

fn find(y: usize, x: usize, table: &Vec<&[u8]>) -> i32 {
    let mut cnt = 0;
    for i in 0..8 {
        let y1 = (y as i32) + DY[i];
        let x1 = (x as i32) + DX[i];
        if y1 < 0 || x1 < 0 || y1 >= table.len() as i32 || x1 >= table[0].len() as i32 {
            continue;
        }
        if table[y1 as usize][x1 as usize] == b'N' || table[y1 as usize][x1 as usize] == b'S' {
            let y2 = y1 + DY[i];
            let x2 = x1 + DX[i];
            if y2 < 0 || x2 < 0 || y2 >= table.len() as i32 || x2 >= table[0].len() as i32 {
                continue;
            }
            if table[y2 as usize][x2 as usize] == b'F' || table[y2 as usize][x2 as usize] == b'T' {
                let y3 = y2 + DY[i];
                let x3 = x2 + DX[i];
                if y3 < 0 || x3 < 0 || y3 >= table.len() as i32 || x3 >= table[0].len() as i32 {
                    continue;
                }
                if table[y3 as usize][x3 as usize] == b'P' || table[y3 as usize][x3 as usize] == b'J' {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}