use std::io::{prelude::*, self, BufWriter};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let n = buffer.trim().parse::<usize>().unwrap();

    let mut idx = 1;

    for i in 666..6666001 {
        if n <= 10000 {
            if i.to_string().contains(&String::from("666")) {
                if n == idx {
                    println!("{}", i);
                    break
                }
                idx += 1
            }
        } else {
            break
        }
    }
}