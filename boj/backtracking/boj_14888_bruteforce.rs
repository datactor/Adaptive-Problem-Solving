// https://www.acmicpc.net/problem/14888

use std::io::{self, prelude::*};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut v = buffer
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>())
        .flatten();

    let n = v.next().unwrap();
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(v.next().unwrap());
    }

    let mut result = Result {
        max: -1000_000_000,
        min: 1000_000_000,
    };

    let mut operators = Vec::new();
    for _ in 0..4 {
        operators.push(v.next().unwrap());
    }

    solve(&nums, &mut operators, nums[0], 1, &mut result);
    println!("{}\n{}", result.max, result.min);
}

struct Result {
    max: i32,
    min: i32,
}

fn solve(nums: &Vec<i32>, operators: &mut Vec<i32>, left: i32, idx: usize, result: &mut Result) {
    if idx == nums.len() {
        result.max = left.max(result.max);
        result.min = left.min(result.min);
        return;
    }
    let right = nums[idx];
    for i in 0..4 {
        if operators[i] == 0 {
            continue;
        }
        operators[i] -= 1;
        let next = operate(i as i32, left, right);
        solve(nums, operators, next, idx + 1, result);
        operators[i] += 1;
    }
}

fn operate(operators: i32, left: i32, right: i32) -> i32 {
    match operators {
        0 => left + right,
        1 => left - right,
        2 => left * right,
        _ => left / right,
    }
}
