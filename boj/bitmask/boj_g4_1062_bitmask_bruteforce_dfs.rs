// https://www.acmicpc.net/problem/1062
// O(2^(k-5) * n * (w - 8)log(w - 8))

use std::{
    io::{self, prelude::*, BufWriter},
};

fn main() -> io::Result<()> {
    let mut input = io::stdin().lock().lines();
    let mut output = BufWriter::new(io::stdout().lock());

    let mut line = || input.next().unwrap().unwrap();
    let first_line = line();
    let (n, k) = first_line.split_once(' ').unwrap();
    let (n, mut k) = (n.parse::<usize>().unwrap(), k.parse::<usize>().unwrap());
    if k < 5 {
        writeln!(output, "{}", 0)?;
        return Ok(());
    }

    k -= 5;
    let mut max_count = 0;
    let mut visited = [false; 26];
    let mut words = Vec::new();
    for _ in 0..n {
        let line = line();
        let mut word = line.as_bytes().to_vec();
        word.sort();
        word.dedup();
        let mut new_word = Vec::new();
        for c in word {
            if c != b'a' && c != b'n' && c != b't' && c != b'i' && c != b'c' {
                new_word.push(c);
            }
        }
        words.push(new_word);
    }

    visited[b'a' as usize - 97] = true;
    visited[b'n' as usize - 97] = true;
    visited[b't' as usize - 97] = true;
    visited[b'i' as usize - 97] = true;
    visited[b'c' as usize - 97] = true;
    dfs(&words, &mut visited, k, 0, &mut max_count);

    writeln!(output, "{}", max_count)?;
    Ok(())
}

fn dfs(words: &[Vec<u8>], visited: &mut [bool], k: usize, start: usize, max_count: &mut usize) {
    if k == 0 {
        let mut count = 0;
        for word in words {
            if word.iter().all(|&c| visited[c as usize - 97]) {
                count += 1;
            }
        }
        *max_count = (*max_count).max(count);
        return;
    }

    for i in start..26 {
        if !visited[i] {
            visited[i] = true;
            dfs(words, visited, k - 1, i + 1, max_count);
            visited[i] = false;
        }
    }
}