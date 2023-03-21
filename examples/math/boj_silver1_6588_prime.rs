// https://www.acmicpc.net/problem/6588

use std::io::{self, prelude::*, BufWriter};

fn main() -> io::Result<()> {
    let mut input = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim().parse::<usize>().unwrap()); // lazy
    let mut output = BufWriter::new(io::stdout().lock());

    let mut table = is_prime_list(1_000_000);
    (table[0], table[1], table[2]) = (true, true, true);

    while let Some(n) = input.next() {
        if n == 0 {
            break
        }

        let mut left = 3;
        let mut right = n - 3;

        while left <= right {
            if !table[left] && !table[right] {
                if left + right == n {
                    break;
                }
            }
            left += 2;
            right -= 2;
        }

        if left > right {
            writeln!(output, "Goldbach's conjecture is wrong.")?;
        } else {
            writeln!(output, "{} = {} + {}", n, left, right)?;
        }
    }

    Ok(())
}

// Sieve of Eratosthenes
fn is_prime_list(n: usize) -> Vec<bool> {
    let mut sieve = vec![false; n + 1];
    let m = (n as f32).sqrt();
    for i in 2..=m as usize {
        if !sieve[i] {
            for j in (2 * i..=n).step_by(i) {
                sieve[j] = true
            }
        }
    }
    return sieve
}
