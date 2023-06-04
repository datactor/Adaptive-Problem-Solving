// https://www.acmicpc.net/problem/1644
// O(N * log(log(N)))

use std::{
    error::Error,
    io::{self, prelude::*},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n = input.trim().parse::<usize>().unwrap();

    let primes = prime_list(n);

    let (mut left, mut right, mut ans) = (0, 0, 0);

    while right <= primes.len() {
        let tmp_sum = primes[left..right].iter().sum::<usize>();
        if tmp_sum == n {
            ans += 1;
            right += 1;
        } else if tmp_sum < n {
            right += 1;
        } else {
            left += 1;
        }
    }
    println!("{}", ans);

    Ok(())
}

fn prime_list(n: usize) -> Vec<usize> {
    // Sieve of Eratosthenes
    let mut sieve = vec![true; n + 1];
    let m = (n as f32).sqrt();
    for i in 2..=m as usize {
        if sieve[i] {
            for j in (2 * i..=n).step_by(i) {
                sieve[j] = false
            }
        }
    }
    return (2..=n)
        .filter_map(|i| (sieve[i] == true).then(|| i))
        .collect::<Vec<_>>();
}
