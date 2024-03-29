// https://www.acmicpc.net/problem/11401
// O(n + lgp)

// Fermat's little Theorem
// p가 소수이면, 모든 정수 a에 대해 a^p ≡ a (mod p)이다.
// 혹은 p가 소수이고 a가 p의 배수가 아니면, a^(p-1) ≡ 1 (mod p)이다.
// 즉, a^(p-1)을 p로 나눈 나머지는 1이다.
//
// 페르마의 소정리에 의해 분모의 수를 아래와 같이 바꿀 수 있다.
// B^p-1 ≡ 1 (mod p)
// B*B^(p-2) ≡ 1 (mod p)
// B^(p-2) ≡ B^(-1) (mod p)
//
// AB^(-1) % p = AB^(p-2)%p = (A%p)*(B^(p-2)%p)%p
// 위의 식을 이항계수 공식에 대입하면
// (n!/k!(n-k)!)%p = n!%p * (k!(n-k)!)^(p-2) % p
//                 = (n!/k!)%p * (n-k)!^(p-2) % p

use std::{
    error::Error,
    io::{self},
};

const P: u64 = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut v = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>())
        .flatten();
    let (n, k) = (v.next().unwrap(), v.next().unwrap());

    let a = fac(k + 1, n);
    let b = fac(2, n - k) % P;

    println!("{}", (a % P) * mod_pow(b, P - 2) % P);
    Ok(())
}

fn fac(start: u64, end: u64) -> u64 {
    let mut result = 1;
    for i in start..=end {
        result = (result * i) % P;
    }
    result
}

fn mod_pow(base: u64, exponent: u64) -> u64 {
    match exponent {
        0 => 1,
        e if e % 2 == 1 => (mod_pow(base, e - 1) * base) % P,
        _ => (mod_pow(base, exponent / 2).pow(2)) % P,
    }
}