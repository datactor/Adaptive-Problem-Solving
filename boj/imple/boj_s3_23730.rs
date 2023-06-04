// https://www.acmicpc.net/problem/23730

use std::{
    io::{self, BufRead},
    fmt::{Write, Error},
};

fn main() -> Result<(), Error> {
    let mut lines = io::stdin().lock().lines();

    let n = lines.next().unwrap().unwrap().split_once(' ').unwrap().0.parse::<usize>().unwrap();

    let mut yunee = vec![0u8; n];
    let mut answer_sheet: Vec<u8> = Vec::with_capacity(n);

    for (i, is_line) in lines.enumerate() {
        let line = is_line.unwrap();
        let iter = line.split_ascii_whitespace();
        if i == 0 {
            answer_sheet = iter.map(|s| s.parse::<u8>().unwrap()).collect();
        } else {
            for i in iter {
                let i = i.parse::<usize>().unwrap() - 1;
                yunee[i] = answer_sheet[i];
            }
        }
    }

    let mut writer = String::new();

    for i in 0..n {
        let mut num = yunee[i];
        if yunee[i] == 0 {
            let answer = answer_sheet[i];
            let prev = yunee.get(i.wrapping_sub(1)).unwrap_or(&answer);
            let next = match yunee.get(i+1) {
                Some(&0) | None => answer,
                Some(&next) => next,
            };
            yunee[i] = (1..6).find(|&j| j != answer && j != *prev && j != next).unwrap();
            num = yunee[i];
        }
        write!(writer, "{} ", num)?;
    }
    print!("{}", writer);

    Ok(())
}