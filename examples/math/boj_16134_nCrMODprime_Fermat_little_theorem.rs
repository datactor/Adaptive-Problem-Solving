// https://www.acmicpc.net/problem/16134
// O(N + lgN)

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
    io::{self, prelude::*, BufWriter},
    error::Error,
};

const P: u64 = 1_000_000_007;

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input:s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let (n, r) = (sc.read::<u64>(), sc.read::<u64>());

    let a = fac(r + 1, n);
    let b = fac(2, n - r) % P;

    writeln!(output, "{}", (a % P) * sqr(b, P - 2) % P)?;

    Ok(())
}

fn fac(s: u64, n: u64) -> u64 {
    let mut tmp = 1;
    for i in s..=n {
        tmp = (tmp * i) % P;
    }
    tmp
}

fn sqr(a: u64, b: u64) -> u64 {
    match b {
        0 => 1,
        b if b % 2 == 1 => (sqr(a, b - 1) * a) % P,
        _ => (sqr(a, b / 2).pow(2)) % P,
    }
}