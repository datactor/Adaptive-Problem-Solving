use std::{
    io::{self, prelude::*, BufWriter},
    error::Error,
    collections::VecDeque,
};

struct Scanner<'a> {
    it: std::str::SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Scanner<'a> {
        Scanner {
            it: s.split_ascii_whitespace(),
        }
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        self.it.next().unwrap().parse::<T>().ok().unwrap()
    }
}

struct Lis {
    arr: Vec<i32>,
    val_and_len: Vec<(i32, usize)>,
}

impl Lis {
    fn new() -> Self {
        Lis {
            arr: vec![-1_000_000_001],
            val_and_len: vec![(-1_000_000_001, 0)],
        }
    }

    fn cal_bisec(&mut self, mut v: VecDeque<i32>) {
        while !v.is_empty() {
            let num = v.pop_front().unwrap();

            if &num > self.arr.last().unwrap() {
                self.val_and_len.push((num, self.arr.len()));
                self.arr.push(num);
            } else {
                // bisec
                let idx = {
                    let mut low = -1;
                    let mut high = self.arr.len() as i32;

                    while low + 1 < high {
                        let mid = (low + high) / 2;
                        if &num > &self.arr[mid as usize] {
                            low = mid as i32
                        } else {
                            high = mid
                        }
                    } high as usize
                };

                self.arr[idx] = num;
                self.val_and_len.push((num, idx));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanner = Scanner::new(&input);
    let n = scanner.read::<usize>();
    let v: VecDeque<i32> =(0..n).map(|_| scanner.read()).collect();

    let mut lis = Lis::new();

    lis.cal_bisec(v);

    let mut result_len = lis.arr.len() - 1;
    let mut result = Vec::new();

    while !lis.val_and_len.is_empty() && result_len > 0 {
        let (num, idx) = lis.val_and_len.pop().unwrap();
        if idx == result_len {
            result.push(num);
            result_len -= 1;
        }
    }

    let mut output = BufWriter::new(io::stdout().lock());
    writeln!(output, "{}", result.len()).unwrap();
    result
        .iter().rev()
        .for_each(|s| write!(output, "{} ", s).unwrap());

    Ok(())
}