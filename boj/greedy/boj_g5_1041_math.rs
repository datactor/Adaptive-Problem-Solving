// https://www.acmicpc.net/problem/1041

use std::{
    io::{self, Read, Write, BufWriter},
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

    fn next<T> (&mut self) -> Result<T, Box<dyn Error>>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Debug,
    {
        self.input.next()
            .ok_or("Reached out of input")?
            .parse::<T>()
            .map_err(|e| format!("{:?}", e).into())
    }
}

macro_rules! ok {
    (()) => {
        {
            let mut buf_writer = BufWriter::new(io::stdout().lock());
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            let mut scanner = Scanner::new(&buffer);
            let n = scanner.next::<i64>()?;
            let dice = (0..6).map(|_| scanner.next().unwrap()).collect::<Vec<i64>>();

            let ans = if n == 1 {
                dice.iter().sum::<i64>() - dice.iter().max().unwrap()
            } else {
                let mn = dice.iter().min().unwrap();

                let mut mn2 = 100;
                for i in 0..5 {
                    match i {
                        0 => for j in 1..5 { mn2 = mn2.min(dice[i] + dice[j]); },
                        1 => for j in [2, 3, 5] { mn2 = mn2.min(dice[i] + dice[j]); },
                        2 => for j in 4..6 { mn2 = mn2.min(dice[i] + dice[j]); },
                        3 => for j in 4..6 { mn2 = mn2.min(dice[i] + dice[j]); },
                        _ => mn2 = mn2.min(dice[i] + dice[5]),
                    }
                }

                let set3 = [
                    dice[0] + dice[1] + dice[2],
                    dice[0] + dice[1] + dice[3],
                    dice[0] + dice[2] + dice[4],
                    dice[0] + dice[3] + dice[4],
                    dice[1] + dice[2] + dice[5],
                    dice[1] + dice[3] + dice[5],
                    dice[2] + dice[4] + dice[5],
                    dice[3] + dice[4] + dice[5]];
                let mn3 = set3.iter().min().unwrap();

                let one = (n - 2).pow(2) * 5 + (n - 2) * 4;
                let two = 4 + (n - 2) * 8;
                let three = 4;

                one * mn + two * mn2 + three * mn3
            };

            write!(buf_writer, "{}", ans)?;
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    ok!(())
}