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

// O(n + lgp)
pub fn fermat_little_theorem(n: u128, r: u128, p: u128) -> u128{
    fn fac(s: u128, n: u128) -> u128 {
        let mut tmp = 1;
        for i in s..n + 1 {
            tmp = (tmp * i) % P;
        }
        tmp
    }

    fn sqr(a: u128, b: u128) -> u128 {
        match b {
            0 => 1,
            b if b % 2 == 1 => (sqr(a, b - 1) * a) % P,
            _ => (sqr(a, b / 2).pow(2)) % P,
        }
    }

    let a = fac(r + 1, n);
    let b = fac(2, n - r) % p;

    return (a % p) * sqr(b, p - 2) % p;
}

