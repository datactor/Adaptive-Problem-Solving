use std::io::{self, prelude::*, BufWriter};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut idx = 1;
    let mut result = 0;

    for i in 666..2666800 {
        if n >= idx {
            let mut x = i;
            while x > 0 {
                if x % 1000 == 666 {
                    idx += 1;
                    result = i;
                    x = 0;
                } else {
                    x /= 10;
                }
            }
        } else {
            break;
        }
    }
    println!("{}", result);
}
