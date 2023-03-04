pub fn combinations<T>(data: &[T], r: usize) -> Vec<Vec<&T>> {
    fn combinations_helper<'a, T>(start: usize, data: &'a [T], r: usize, comb: &mut Vec<&'a T>, result: &mut Vec<Vec<&'a T>>) {
        if r == 0 {
            result.push(comb.clone());
        } else if start < data.len() {
            comb.push(&data[start]);
            combinations_helper(start + 1, data, r - 1, comb, result);
            comb.pop();
            combinations_helper(start + 1, data, r, comb, result);
        }
    }

    let mut result = vec![];
    let mut comb = vec![];
    combinations_helper(0, data, r, &mut comb, &mut result);
    result
}

/// Fermat's little Theorem - `O(n + lgp)`
///
/// p가 소수이면, 모든 정수 a에 대해 a^p ≡ a (mod p)이다.
///
/// 혹은 p가 소수이고 a가 p의 배수가 아니면, a^(p-1) ≡ 1 (mod p)이다.
///
/// 즉, a^(p-1)을 p로 나눈 나머지는 1이다.
///
/// 페르마의 소정리에 의해 분모의 수를 아래와 같이 바꿀 수 있다.
///
/// 1. B^p-1 ≡ 1 (mod p)
/// 2. B*B^(p-2) ≡ 1 (mod p)
/// 3. B^(p-2) ≡ B^(-1) (mod p)
/// 4. AB^(-1) % p = AB^(p-2)%p = (A%p)*(B^(p-2)%p)%p
///
/// 위의 식을 이항계수 공식에 대입하면
///
/// (n!/k!(n-k)!)%p = n!%p * (k!(n-k)!)^(p-2) % p
///
///                 = (n!/k!)%p * (n-k)!^(p-2) % p
pub fn fermat_little_theorem(n: u64, r: u64, p: u64) -> u64 {
    fn fac(start: u64, end: u64) -> u64 {
        let mut result = 1;
        for i in start..=end {
            result = (result * i) % p;
        }
        result
    }

    fn mod_pow(base: u64, exponent: u64) -> u64 {
        match exponent {
            0 => 1,
            e if e % 2 == 1 => (mod_pow(base, e - 1) * base) % p,
            _ => (mod_pow(base, exponent / 2).pow(2)) % p,
        }
    }

    let numerator = fac(r + 1, n);
    let denominator = fac(2, n - r) % p;

    (numerator % p) * mod_pow(denominator, p - 2) % p
}