use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut output = BufWriter::new(io::stdout().lock());
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let (n, m) = (scanner.read::<usize>(), scanner.read::<usize>());

    let mut result = vec![0; n];
    let answers: Vec<usize> = (0..n).map(|_| scanner.read::<usize>()).collect();

    for _ in 0..m {
        let tmp = scanner.read::<usize>()-1;
        result[tmp] = answers[tmp]
    }

    if n == 2 {
        if result[0] == 0 {
            for i in 1..=5 {
                if result[1] != i && answers[0] != i {
                    result[0] = i;
                    break
                }
            }
        }
        if result[1] == 0 {
            for i in 1..=5 {
                if result[0] != i && answers[1] != i {
                    result[1] = i;
                    break
                }
            }
        }
    }

    if n > 2 {
        if result[0] == 0 {
            for i in 1..=5 {
                if result[1] != i && answers[0] != i {
                    result[0] = i;
                    break
                }
            }
        }

        for i in 1..n-1 {
            if result[i] == 0 {
                for j in 1..=5 {
                    if result[i-1] != j && result[i+1] != j && answers[i] != j {
                        result[i] = j;
                        break
                    }
                }
            }
        }

        if result[n-1] == 0 {
            for i in 1..=5 {
                if result[n-2] != i {
                    result[n-1] = i;
                    break
                }
            }
        }
    }

    for i in &result {
        write!(output, "{} ", i)?;
    }

    Ok(())
}
