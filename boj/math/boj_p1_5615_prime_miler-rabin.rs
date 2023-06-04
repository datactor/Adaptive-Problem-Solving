// https://www.acmicpc.net/problem/5615
// O(n log^2 num)

use std::{
    io::{self, Write, BufRead, BufWriter},
    collections::HashSet,
};

fn miller_rabin(n: u64, primeset: &HashSet<u64>) -> bool {
    if primeset.contains(&n) {
        return true;
    }
    if n == 1 || n % 2 == 0 {
        return false;
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'next_a: for &a in primeset {
        let mut y = pow_mod(a, d, n);
        if y == 1 || y == n - 1 {
            continue 'next_a;
        }
        for _r in 0..s {
            y = (y * y) % n;
            if y == n - 1 {
                continue 'next_a;
            }
        }
        return false;
    }
    true
}

fn pow_mod(x: u64, y: u64, z: u64) -> u64 {
    let mut n = 1;
    let mut p = x % z;
    let mut y = y;
    while y > 0 {
        if y % 2 == 1 {
            n = (n * p) % z;
        }
        p = (p * p) % z;
        y /= 2;
    }
    n
}

fn main() -> io::Result<()> {
    let mut cnt = 0;
    let primeset: HashSet<u64> = [2, 7, 61].iter().cloned().collect();
    for line in io::stdin().lock().lines().skip(1) {
        let s: u64 = line?.trim().parse().unwrap();
        if miller_rabin(2 * s + 1, &primeset) {
            cnt += 1;
        }
    }
    writeln!(BufWriter::new(io::stdout().lock()), "{}", cnt)?;
    Ok(())
}