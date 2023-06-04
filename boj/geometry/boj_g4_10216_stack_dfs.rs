// https://www.acmicpc.net/problem/10216
// O(tn^2)

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut output = BufWriter::new(io::stdout().lock());
    let t: usize = input.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let n: usize = input.next().unwrap().trim().parse().unwrap();
        let camps = (0..n)
            .map(|_| {
                let nums: Vec<i32> = input
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (nums[0], nums[1], nums[2])
            })
            .collect::<Vec<(i32, i32, i32)>>();

        let mut groups = 0;
        let mut visited = vec![false; n];

        for i in 0..n {
            if !visited[i] {
                // Start a new group
                groups += 1;
                visited[i] = true;
                let mut stack = vec![i];

                while let Some(idx) = stack.pop() {
                    // Add all unvisited overlapping camps to the group
                    for j in 0..n {
                        if !visited[j] && overlap(&camps[idx], &camps[j]) {
                            visited[j] = true;
                            stack.push(j);
                        }
                    }
                }
            }
        }
        writeln!(output, "{}", groups)?;
    }

    Ok(())
}

fn overlap(c1: &(i32, i32, i32), c2: &(i32, i32, i32)) -> bool {
    (c1.0 - c2.0).pow(2) + (c1.1 - c2.1).pow(2) <= (c1.2 + c2.2).pow(2)
}
