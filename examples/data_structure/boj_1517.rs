// https://www.acmicpc.net/problem/1517
// O(N * lgN)

use std::{
    io::{self, prelude::*},
    error::Error,
};

struct Scanner<'a> {
    input: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_ascii_whitespace(),
        }
    }

    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut sc = Scanner::new(&input);

    let n = sc.read::<usize>();

    let mut arr: Vec<usize> = (0..n).map(|_| sc.read::<usize>()).collect();

    let mut swap = 0;

    // // 실제 bubble sort
    // loop {
    //     let mut tmp_swap = 0;
    //     for i in 0..n-1 {
    //         match v[i].cmp(&arr[i+1]) {
    //             Ordering::Greater => {
    //                 swap += 1;
    //                 tmp_swap += 1;
    //                 arr[i..i+2].rotate_left(1)
    //             },
    //             _ => {},
    //         }
    //     }
    //     if tmp_swap == 0 {
    //         break
    //     }
    // }
    // 다른 방식의 bubble sort는 현재 자리의 수부터 차례로 뒤의 수를 brute forcing 했을 때,
    // 현재의 수를 계속해서 뒤로 옮기는 방식. 이 방식은 뒤에 있는 원소들 중 자신보다 낮은 수 개수만큼 연산을 함.

    // 분할정렬
    merge_sort(0, n, &mut arr, &mut swap);

    println!("{}", swap);

    Ok(())
}

fn merge_sort(start: usize, end: usize, arr: &mut Vec<usize>, swap: &mut i64) {
    let size = end - start;
    let mid = (start + end) / 2;
    if size <= 1 {
        return;
    }

    // divide
    merge_sort(start, mid, arr, swap);
    merge_sort(mid, end, arr, swap);

    // merge
    let mut new_arr = Vec::with_capacity(size);
    let (mut idx1, mut idx2) = (start, mid);
    let mut cnt = 0;

    while idx1 < mid && idx2 < end {
        if arr[idx1] > arr[idx2] {
            new_arr.push(arr[idx2]);
            idx2 += 1;
            cnt += 1;
        } else {
            new_arr.push(arr[idx1]);
            idx1 += 1;
            *swap += cnt;
        }
    }

    while idx1 < mid {
        new_arr.push(arr[idx1]);
        idx1 += 1;
        *swap += cnt;
    }

    while idx2 < end {
        new_arr.push(arr[idx2]);
        idx2 += 1;
    }
    // reflect
    arr[start..end].copy_from_slice(&new_arr[..]);
}