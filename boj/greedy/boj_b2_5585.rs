// https://www.acmicpc.net/problem/5585

use std::{
    io::{self, BufRead, Write},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut change = 1000 - buffer.trim().parse::<i32>()?;

    let table = [500, 100, 50, 10, 5, 1];

    let mut num = 0;
    for coin in table {
        if change >= coin {
            let tmp = change / coin;
            num += tmp;
            change -= tmp * coin;
        }
    }

    write!(io::stdout(), "{}", num)?;
    Ok(())
}