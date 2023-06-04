// https://www.acmicpc.net/problem/27965

use std::io::{self, Write, BufRead, BufWriter};

// // 1차 시도
// fn main() -> io::Result<()> {
//     let mut input = io::stdin().lock().lines().next().unwrap().unwrap();
//
//     let mut buf = input.split_ascii_whitespace();
//     let n: i64 = buf.next().unwrap().parse().unwrap();
//     let k: i64 = buf.next().unwrap().parse().unwrap();
//
//     let mut ans: i64 = 0;
//     for i in 1..=n {
//         let leng = (i as f64).log10().floor() as i64 + 1;
//         let mut leng_copy = leng;
//         while leng_copy > 0 {
//             ans *= 10;
//             leng_copy -= 1;
//         }
//         ans += i;
//         ans %= k;
//     }
//     writeln!(BufWriter::new(io::stdout().lock()), "{}", ans)?;
//     Ok(())
// }

fn main() -> io::Result<()> {
    let input = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut buf = input.split_whitespace();
    let n: i64 = buf.next().unwrap().parse().unwrap();
    let k: i64 = buf.next().unwrap().parse().unwrap();

    let mut tens_powers = [0i64; 10];
    tens_powers[0] = 1;
    for i in 1..=8 {
        tens_powers[i] = tens_powers[i - 1] * 10;
    }

    let mut ten1000_powers = [0i64; 10];
    ten1000_powers[1] = 1;
    for _ in 0..1000 {
        ten1000_powers[1] = (ten1000_powers[1] * 10) % k;
    }
    for i in 2..=8 {
        ten1000_powers[i] = (ten1000_powers[i - 1] * ten1000_powers[1]) % k;
    }

    let mut ans: i64 = 0;
    let mut ten: usize = 1;
    for i in 1..=std::cmp::min(999, n) {
        if i == tens_powers[ten] {
            ten += 1;
        }
        ans = (ans * tens_powers[ten] + i) % k;
    }

    writeln!(BufWriter::new(io::stdout().lock()), "{}",
        if n < 1000 {
            ans
        } else {
            let mut mul = [0i64; 10];
            let mut add = [0i64; 10];
            for i in 3..=8 {
                for j in 0..1000 {
                    mul[i] = (mul[i] * tens_powers[i] + 1) % k;
                    add[i] = (add[i] * tens_powers[i] + j) % k;
                }
            }

            let mut i = 1000;
            while i <= n - 999 {
                if i == tens_powers[ten] {
                    ten += 1;
                }
                ans = (ans * ten1000_powers[ten] % k + mul[ten] * i % k + add[ten]) % k;
                i += 1000;
            }

            while i <= n {
                if i == tens_powers[ten] {
                    ten += 1;
                }
                ans = (ans * tens_powers[ten] + i) % k;
                i += 1;
            }

            ans
        }
    )?;
    Ok(())
}