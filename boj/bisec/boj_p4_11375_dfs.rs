// https://www.acmicpc.net/problem/11375

use std::{
    io::{self, prelude::*, BufReader, BufWriter},
    str::{FromStr, SplitAsciiWhitespace},
    num::{ParseIntError as IE, ParseFloatError as FE},
    fmt,
    collections::VecDeque,
};

trait Parser {
    fn read<T, E>(&mut self) -> T where T : FromStr<Err = E>,  E : fmt::Debug;
}

impl<'a> Parser for SplitAsciiWhitespace<'a> {
    fn read<T, E>(&mut self) -> T
        where
            T: FromStr<Err = E>,
            E: fmt::Debug,
    {
        match self.next() {
            Some(value) => value.parse().expect("Parse Error"),
            None => panic!("Unexpected EOF"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let mut iter = buffer.split_ascii_whitespace();
    let (n, m): (usize, usize) = (iter.read(), iter.read());

    let mut task_dependency = vec![vec![false; m]; n];
    let mut task_count = vec![0; n];
    let mut worker_count = vec![0; m];
    let mut task_queue = VecDeque::new();

    for i in 0..n {
        buffer.clear();
        reader.read_line(&mut buffer)?;
        let mut iter = buffer.split_ascii_whitespace();

        let task_len: usize = iter.read();
        task_count[i] = task_len;

        for _ in 0..task_len {
            let task = iter.read::<usize, IE>() - 1;
            task_dependency[i][task] = true;
            worker_count[task] += 1;
        }
    }

    for i in 0..n {
        if task_count[i] == 1 {
            task_queue.push_back(i);
        }
    }
    for i in 0..m {
        if worker_count[i] == 1 {
            task_queue.push_back(i + 1000);
        }
    }

    let mut assigned = 0;
    while let Some(mut task) = task_queue.pop_front() {
        if task < 1000 {
            if task_count[task] == 0 {
                continue;
            }
            let worker = task_dependency[task].iter().position(|&x| x).unwrap();
            assigned += 1;
            worker_count[worker] = 0;
            for i in 0..n {
                if task_dependency[i][worker] {
                    task_count[i] -= 1;
                    task_dependency[i][worker] = false;
                    if task_count[i] == 1 {
                        task_queue.push_back(i);
                    }
                }
            }
            continue;
        }
        task -= 1000;
        if worker_count[task] == 0 {
            continue;
        }
        let task = task_dependency.iter().position(|row| row[task]).unwrap();
        task_count[task] = 0;
        assigned += 1;
        for i in 0..m {
            if task_dependency[task][i] {
                worker_count[i] -= 1;
                task_dependency[task][i] = false;
                if worker_count[i] == 1 {
                    task_queue.push_back(i + 1000);
                }
            }
        }
    }

    writeln!(writer, "{}",
        assigned + std::cmp::min(
            n - task_count.iter().filter(|&&x| x == 0).count(),
            m - worker_count.iter().filter(|&&x| x == 0).count()
        )
    )?;

    Ok(())
}