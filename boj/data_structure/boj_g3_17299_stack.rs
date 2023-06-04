// https://www.acmicpc.net/problem/17299
// O(n)

use std::{
    io::{self, prelude::*, BufWriter},
    collections::HashMap,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().lock().read_to_string(&mut input)?;

    let mut input = input.split_ascii_whitespace();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let a: Vec<i32> = input.map(|s| s.parse::<i32>().unwrap()).collect();

    let mut freq = HashMap::new();
    for &i in &a {
        *freq.entry(i).or_insert(0) += 1;
    }

    let mut ans = vec![-1; n];
    let mut stack = Vec::new();
    println!("{:?}", freq);

    for i in 0..n {
        while let Some(last) = stack.last() {
            // stack의 마지막 인덱스에 해당하는 a의 값이 a에 몇개 있는지 확인,
            // 현재 인덱스인 i에 해당하는 a의 값이 더 크면 ans를 업데이트하고 stack에서 pop.
            if freq[&a[*last]] < freq[&a[i]] {
                ans[*last] = a[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
        println!("ans: {:?}, stack: {:?}", ans, stack);
    }

    for i in ans {
        write!(output, "{} ", i)?;
    }

    Ok(())
}