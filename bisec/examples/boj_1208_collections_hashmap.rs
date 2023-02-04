use std::{
    io::{self, prelude::*},
    error::Error,
    collections::HashMap,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner {
        Scanner {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn combinations<T>(data: &[T], r: usize) -> Vec<Vec<&T>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, s) = (scanner.read::<usize>(), scanner.read::<i32>());

    // hashmap과 부분집합, collections를 만들어서 풀기
    let left: Vec<i32> = (0..n/2).map(|_| scanner.read::<i32>()).collect();
    let mut last = n/2;
    if n % 2 == 1 {
        last += 1;
    }
    let right: Vec<i32> = (0..last).map(|_| scanner.read::<i32>()).collect();

    let mut left_subset = HashMap::with_capacity(n/2);
    let mut right_subset = HashMap::with_capacity(n-n/2);

    for r in 1..left.len()+1 {
        for subset in combinations(&left, r) {
            let tmp_sum = subset.iter().cloned().sum::<i32>();
            *left_subset.entry(tmp_sum).or_insert(0) += 1;
        }
    }

    for r in 1..right.len()+1 {
        for subset in combinations(&right, r) {
            let tmp_sum = subset.iter().cloned().sum::<i32>();
            *right_subset.entry(tmp_sum).or_insert(0) += 1;
        }
    }


    let mut count = *left_subset.get(&s).unwrap_or(&0) as usize + *right_subset.get(&s).unwrap_or(&0) as usize;

    for (k, _v) in &left_subset {
        if right_subset.contains_key(&(s - k)) {
            count += left_subset[k] * right_subset[&(s - k)]
        }
    }

    // // bisec으로 풀기
    // let mut left_sums_of_subsets = vec![0];
    //
    // for num in (0..n/2).map(|_| scanner.read::<i32>()) {
    //     for i in 0..left_sums_of_subsets.len() {
    //         left_sums_of_subsets.push(left_sums_of_subsets[i] + num);
    //     }
    // }
    // left_sums_of_subsets.sort();
    //
    // let mut prev = left_sums_of_subsets[0];
    // let mut cnt = 0;
    // let mut left_cnts = vec![];
    // for left_i in left_sums_of_subsets {
    //     if prev == left_i {
    //         cnt += 1;
    //     } else {
    //         left_cnts.push((prev, cnt));
    //         prev = left_i;
    //         cnt = 1;
    //     }
    // }
    // left_cnts.push((prev, cnt));
    //
    // let mut right_sums_of_subsets = vec![0];
    // for num in scanner.input.into_iter().map(|s| s.parse::<i32>().unwrap()) {
    //     for i in 0..right_sums_of_subsets.len() {
    //         right_sums_of_subsets.push(right_sums_of_subsets[i] + num);
    //     }
    // }
    // right_sums_of_subsets.sort();
    //
    // let mut prev = right_sums_of_subsets[0];
    // let mut cnt = 0usize;
    // let mut right_cnts = vec![];
    // for right_i in right_sums_of_subsets {
    //     if prev == right_i {
    //         cnt += 1;
    //     } else {
    //         right_cnts.push((prev, cnt));
    //         prev = right_i;
    //         cnt = 1;
    //     }
    // }
    //
    // right_cnts.push((prev, cnt));
    //
    // let mut li = 0;
    // let mut re = right_cnts.len();
    // let mut count = 0;
    // while li < left_cnts.len() && re > 0 {
    //     let sum = left_cnts[li].0 + right_cnts[re - 1].0;
    //     if sum == s {
    //         count += left_cnts[li].1 * right_cnts[re - 1].1;
    //         re -= 1;
    //     } else if sum > s {
    //         re -= 1;
    //     } else {
    //         li += 1;
    //     }
    // }
    // if s == 0 {
    //     count -= 1;
    // }
    //


    println!("{}", count);
    Ok(())
}