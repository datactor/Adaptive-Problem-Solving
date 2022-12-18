use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut output = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer.split_ascii_whitespace().skip(1).map(
        |s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    v.sort();

    let n = v.len();

    let mut subset = Vec::with_capacity(n-1);
    for i in 0..n-1 {
        subset.insert(i, v[i+1] - v[i]);
    }

    let mut gcd = subset[0];
    for i in 1..n-1 {
        gcd = find_gcd(gcd, subset[i]);
    }
    v.clear();

    let mut set = Vec::new();
    for i in 2..(gcd as f32).powf(0.5) as usize + 1 {
        if gcd % i == 0 {
            set.push(i);
            set.push(gcd / i);
        }
    }
    set.push(gcd);
    set.dedup();
    set.sort();

    for i in set {
        write!(output, "{} ", i).unwrap();
    }

    // let mut tmp = v[0];
    // tmp.max((v[1] as f32).powf(0.5).ceil() as usize);
    //
    // let mut x = HashSet::new();
    // for i in 2..tmp {
    //     let mut res = 0;
    //     for j in &v {
    //         if j == &v[0] {
    //             res = j % i;
    //             continue
    //         } else if j % i != res {
    //             break
    //         }
    //         if j == &v[n-1] {
    //             x.insert(i);
    //             // write!(output, "{} ", i).unwrap();
    //         }
    //     }
    // }
    // let mut result = x.iter().map(|s| s).collect::<Vec<&usize>>();
    // result.sort();
    //
    // for i in result {
    //     write!(output, "{} ", i).unwrap();
    // }
}

fn find_gcd(mut div: usize, mut num: usize) -> usize {
    let mut res = num % div;
    while res != 0 {
        num = div;
        div = res;
        res = num % div;
    }
    div
}