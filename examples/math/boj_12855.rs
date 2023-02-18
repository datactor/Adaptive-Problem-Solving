// use std::{
//     io::{self, prelude::*, BufWriter},
//     error::Error,
//     cmp,
// };
//
// const MOD: i64 = 1000_000_007;
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut input = String::new();
//     let mut output = BufWriter::new(io::stdout().lock());
//     io::stdin().read_line(&mut input)?;
//
//     let mut input = input.split_ascii_whitespace().map(|s| s.parse::<i64>()).flatten();
//     let (n, d, x) = (input.next().unwrap(), input.next().unwrap(), input.next().unwrap());
//
//     let (a, b) = init_ab(n, d, x);
//
//     println!("{:?}, {:?}", a, b);
//
//     let x = vec![2, 3, 1];
//     let y = vec![0, 1, 0];
//
//
//     println!("{:?}", cal_c(&*x, &*y));
//     println!("{:?}", cal_c(&*x, &*y));
//
//     Ok(())
// }
//
// fn get_next_x(x: &mut i64) -> i64 {
//     *x = (*x * 37 + 10007) % MOD;
//     *x
// }
//
// fn init_ab(n: i64, d: i64, x: i64) -> (Vec<i64>, Vec<i64>) {
//     let mut a = vec![0; n as usize];
//     let mut b = vec![0; n as usize];
//     for i in 0..n {
//         a[i as usize] = i + 1;
//     }
//     for i in 0..n {
//         let j = (get_next_x(&mut (x as i64)) % cmp::max(i + 1, 1)) as usize;
//         a.swap(i as usize, j);
//     }
//     for i in 0..n {
//         b[i as usize] = if i < d { 1 } else { 0 };
//     }
//     for i in 0..n {
//         let j = (get_next_x(&mut (x as i64)) % cmp::max(i + 1, 1)) as usize;
//         b.swap(i as usize, j);
//     }
//     return (a, b)
// }
//
//
// fn cal_c(a: &[i64], b: &[i64]) -> Vec<i64> {
//     let n = a.len();
//     let mut c = vec![0; n];
//     for i in 0..n {
//         for j in 0..=i {
//             if b[i - j] != 0 {
//                 c[i] = cmp::max(c[i], a[j] * b[i - j]);
//             }
//         }
//     }
//     c
// }