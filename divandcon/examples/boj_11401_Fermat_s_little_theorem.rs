// https://www.acmicpc.net/problem/11401
// O(N + lnN)

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
    io::{self},
    error::Error,
};

static P: u128 = 1_000_000_007;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut v = input.split_ascii_whitespace().map(|s| s.parse::<u128>()).flatten();
    let (n, k) = (v.next().unwrap(), v.next().unwrap());

    let a = fac(k+1, n);
    let b = fac(2, n-k) % P;

    let x = ((1 / b) as f64) as u128;
    println!("{}", (a%P) * sqr(b, P-2) % P);
    Ok(())
}

fn fac(mut s: u128, mut n: u128) -> u128{
    let mut tmp = 1;
    for i in s..n+1 {
        tmp = (tmp * i) % P;
    } tmp
}

fn sqr(a: u128, b: u128) -> u128 {
    match b {
        0 => 1,
        b if b % 2 == 1 => (sqr(a, b-1) * a) % P,
        _ => (sqr(a, b/2).pow(2)) % P,
    }
}